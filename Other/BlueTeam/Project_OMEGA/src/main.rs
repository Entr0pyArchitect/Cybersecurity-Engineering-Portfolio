use aes_gcm::{Aes256Gcm, Nonce as AesNonce};
use aes_gcm_siv::{Aes256GcmSiv, Nonce as AesSivNonce};
use anyhow::{anyhow, bail, Context, Result};
use arboard::Clipboard;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use clap::{Parser, Subcommand, ValueEnum};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use rand_chacha::ChaCha20Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use zeroize::{Zeroize, Zeroizing};

const DEFAULT_LENGTH: usize = 48;
const SALT_LEN: usize = 16;
const KDF_MEMORY_KIB: u32 = 64 * 1024;
const KDF_ITERATIONS: u32 = 3;
const KDF_LANES: u32 = 1;
const KDF_OUTPUT_LEN: usize = 32;
const MIN_FILE_PASSWORD_LEN: usize = 12;
const HISTORY_DOMAIN: &[u8] = b"AegisPhrase local no-repeat history v2\0";
const RNG_DOMAIN: &[u8] = b"AegisPhrase CSPRNG seed mixer v2\0";
const FILE_FORMAT: &str = "AEGISPHRASE-ENC-V2";
const LEGACY_FILE_FORMAT: &str = "AEGISPHRASE-ENC-V1";

#[derive(Parser, Debug)]
#[command(name = "aegisphrase")]
#[command(
    version,
    about = "Project OMEGA high-entropy passphrase generator, AEAD encryption, and OPSEC hygiene toolkit."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Launch the interactive Protocol OMEGA menu.
    Menu,

    /// Generate a new high-entropy passphrase.
    Gen(GenArgs),

    /// Decrypt an AegisPhrase encrypted passphrase file.
    Decrypt(DecryptArgs),

    /// Encrypt an arbitrary file into an authenticated .apv envelope.
    EncryptFile(FileEncryptArgs),

    /// Decrypt an arbitrary .apv file back to bytes.
    DecryptFile(FileDecryptArgs),

    /// Print and optionally verify the SHA-256 hash of this executable.
    Doctor(DoctorArgs),

    /// Best-effort cleanup of shell command history and terminal screen.
    CleanHistory(CleanHistoryArgs),

    /// Check TPM availability and print the safe integration status.
    TpmStatus,

    /// Print the Windows/Linux readiness checklist status.
    Checklist,
}

#[derive(clap::Args, Debug)]
struct GenArgs {
    /// Length of generated secret, from 30 to 64 characters.
    #[arg(short, long, default_value_t = DEFAULT_LENGTH, value_parser = parse_length)]
    length: usize,

    /// Character set used for generation.
    #[arg(long, value_enum, default_value_t = CharsetChoice::Safe)]
    charset: CharsetChoice,

    /// Copy generated secret to clipboard.
    #[arg(long)]
    copy: bool,

    /// Prompt for hidden user-supplied entropy and mix it with OS randomness.
    #[arg(long)]
    extra_entropy: bool,

    /// Save generated secret into an encrypted .apv file.
    #[arg(long, value_name = "FILE")]
    save: Option<PathBuf>,

    /// AEAD cipher for encrypted save files.
    #[arg(long, value_enum, default_value_t = CipherChoice::Aes256Gcm)]
    cipher: CipherChoice,

    /// Do not print the generated secret to the terminal.
    #[arg(long)]
    hide: bool,

    /// Disable local no-repeat history tracking. Not recommended.
    #[arg(long)]
    no_history: bool,

    /// Custom path for local no-repeat history fingerprints.
    #[arg(long, value_name = "FILE")]
    history_file: Option<PathBuf>,

    /// Explicitly zeroize owned secret buffers before exit where practical.
    #[arg(long)]
    secure_exit: bool,

    /// Clear clipboard immediately before exit if --copy was used.
    #[arg(long)]
    clear_clipboard_on_exit: bool,

    /// Keep process alive for N seconds, then clear clipboard before exit.
    #[arg(long, value_name = "SECONDS")]
    clipboard_ttl: Option<u64>,
}

#[derive(clap::Args, Debug)]
struct DecryptArgs {
    /// Encrypted passphrase file to decrypt.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Copy decrypted secret to clipboard.
    #[arg(long)]
    copy: bool,

    /// Do not print decrypted secret to the terminal.
    #[arg(long)]
    hide: bool,

    /// Clear clipboard immediately before exit if --copy was used.
    #[arg(long)]
    clear_clipboard_on_exit: bool,

    /// Keep process alive for N seconds, then clear clipboard before exit.
    #[arg(long, value_name = "SECONDS")]
    clipboard_ttl: Option<u64>,

    /// Explicitly zeroize owned secret buffers before exit where practical.
    #[arg(long)]
    secure_exit: bool,
}

#[derive(clap::Args, Debug)]
struct FileEncryptArgs {
    /// File to encrypt.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output encrypted .apv file. Defaults to INPUT.apv.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// AEAD cipher for encrypted file.
    #[arg(long, value_enum, default_value_t = CipherChoice::Aes256Gcm)]
    cipher: CipherChoice,

    /// Allow overwriting output file.
    #[arg(long)]
    force: bool,
}

#[derive(clap::Args, Debug)]
struct FileDecryptArgs {
    /// Encrypted .apv file to decrypt.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output plaintext file.
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// Allow overwriting output file.
    #[arg(long)]
    force: bool,
}

#[derive(clap::Args, Debug)]
struct DoctorArgs {
    /// Known-good SHA-256 hash to compare against the running executable.
    #[arg(long, value_name = "SHA256")]
    expected_sha256: Option<String>,
}

#[derive(clap::Args, Debug)]
struct CleanHistoryArgs {
    /// Actually delete shell history files. Without this flag, only clear screen and print paths.
    #[arg(long)]
    yes: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum CharsetChoice {
    /// Strong default: removes common ambiguous characters, keeps symbols.
    Safe,
    /// Full printable ASCII excluding space.
    Full,
    /// URL-safe base64 alphabet. Good for systems that dislike symbols.
    Base64Url,
    /// Hex characters only. Compatible, but lower entropy per character.
    Hex,
}

impl CharsetChoice {
    fn bytes(self) -> &'static [u8] {
        match self {
            CharsetChoice::Safe => b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789!@#$%^&*-_=+?",
            CharsetChoice::Full => b"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            CharsetChoice::Base64Url => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
            CharsetChoice::Hex => b"0123456789abcdef",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum CipherChoice {
    /// NIST-aligned AES-256-GCM authenticated encryption.
    #[value(name = "aes256-gcm")]
    Aes256Gcm,

    /// XChaCha20-Poly1305 AEAD with a 192-bit nonce.
    #[value(name = "xchacha20-poly1305")]
    XChaCha20Poly1305,

    /// AES-256-GCM-SIV authenticated encryption; more nonce-misuse resistant.
    #[value(name = "aes256-gcm-siv")]
    Aes256GcmSiv,
}

impl CipherChoice {
    fn wire_name(self) -> &'static str {
        match self {
            CipherChoice::Aes256Gcm => "aes256-gcm",
            CipherChoice::XChaCha20Poly1305 => "xchacha20-poly1305",
            CipherChoice::Aes256GcmSiv => "aes256-gcm-siv",
        }
    }

    fn from_wire_name(value: &str) -> Result<Self> {
        match value {
            "aes256-gcm" => Ok(Self::Aes256Gcm),
            "xchacha20-poly1305" => Ok(Self::XChaCha20Poly1305),
            "aes256-gcm-siv" => Ok(Self::Aes256GcmSiv),
            other => bail!("unsupported cipher in file: {other}"),
        }
    }

    fn nonce_len(self) -> usize {
        match self {
            CipherChoice::Aes256Gcm | CipherChoice::Aes256GcmSiv => 12,
            CipherChoice::XChaCha20Poly1305 => 24,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedEnvelope {
    format: String,
    version: u8,
    cipher: String,
    kdf: KdfMetadata,
    salt_b64: String,
    nonce_b64: String,
    ciphertext_b64: String,
    created_utc: String,
    kind: Option<String>,
    source_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KdfMetadata {
    name: String,
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
    output_len: usize,
}

fn ui_line(label: &str, message: &str, color: Color) {
    println!("{}", format!("{label} {message}").with(color));
}

fn ui_info(message: &str) {
    ui_line("[*]", message, Color::Cyan);
}

fn ui_success(message: &str) {
    ui_line("[+]", message, Color::Green);
}

fn ui_warn(message: &str) {
    ui_line("[!]", message, Color::Yellow);
}

fn ui_error(message: &str) {
    ui_line("[-]", message, Color::Red);
}

fn ui_section(title: &str) {
    println!();
    println!("{}", format!("[*] {title}").with(Color::Magenta));
    println!("{}", "-".repeat(76).with(Color::DarkGrey));
}

fn ui_menu_item(number: &str, title: &str, detail: &str) {
    println!(
        "{} {} {}",
        format!("[{number}]").with(Color::Green),
        title.with(Color::White),
        detail.with(Color::DarkGrey)
    );
}

fn clear_screen() -> Result<()> {
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Menu) | None => run_menu(),
        Some(Commands::Gen(args)) => run_gen(args),
        Some(Commands::Decrypt(args)) => run_decrypt(args),
        Some(Commands::EncryptFile(args)) => run_encrypt_file(args),
        Some(Commands::DecryptFile(args)) => run_decrypt_file(args),
        Some(Commands::Doctor(args)) => run_doctor(args),
        Some(Commands::CleanHistory(args)) => run_clean_history(args),
        Some(Commands::TpmStatus) => run_tpm_status(),
        Some(Commands::Checklist) => run_checklist_status(),
    }
}

fn run_gen(args: GenArgs) -> Result<()> {
    let extra_entropy = if args.extra_entropy {
        Some(prompt_extra_entropy()?)
    } else {
        None
    };

    let history_path = if args.no_history {
        ui_warn("No-repeat history disabled; local uniqueness cannot be enforced.");
        None
    } else {
        Some(resolve_history_path(args.history_file.as_ref())?)
    };

    let mut secret = generate_unique_secret(
        args.length,
        args.charset.bytes(),
        extra_entropy.as_ref(),
        history_path.as_deref(),
    )?;

    if !args.hide {
        println!("Generated passphrase:\n{}", secret.as_str());
    } else {
        ui_success("Generated passphrase hidden from terminal output.");
    }

    let clipboard = if args.copy {
        let clipboard = copy_to_clipboard(secret.as_str())?;
        ui_success("Copied to clipboard.");
        Some(clipboard)
    } else {
        None
    };

    if let Some(path) = args.save.as_ref() {
        save_encrypted_secret(path, secret.as_str(), args.cipher)?;
        ui_success(&format!("Encrypted file written: {}", path.display()));
    }

    handle_clipboard_exit(clipboard, args.clear_clipboard_on_exit, args.clipboard_ttl)?;

    if args.secure_exit {
        secret.zeroize();
    }

    Ok(())
}

fn run_decrypt(args: DecryptArgs) -> Result<()> {
    let encrypted = fs::read(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let envelope: EncryptedEnvelope = serde_json::from_slice(&encrypted).with_context(|| {
        format!(
            "failed to parse encrypted envelope {}",
            args.input.display()
        )
    })?;

    let password = prompt_file_password("File encryption password: ")?;
    let mut plaintext = decrypt_envelope_to_bytes(&envelope, password.as_bytes())?;
    let plaintext_bytes = std::mem::take(&mut *plaintext);
    let mut secret = Zeroizing::new(
        String::from_utf8(plaintext_bytes)
            .context("decrypted content was not valid UTF-8; use decrypt-file for binary files")?,
    );
    let secret_text = secret.trim_end_matches('\n').to_string();
    secret.zeroize();
    secret = Zeroizing::new(secret_text);

    if !args.hide {
        println!("Decrypted passphrase:\n{}", secret.as_str());
    } else {
        ui_success("Decrypted passphrase hidden from terminal output.");
    }

    let clipboard = if args.copy {
        let clipboard = copy_to_clipboard(secret.as_str())?;
        ui_success("Copied to clipboard.");
        Some(clipboard)
    } else {
        None
    };

    handle_clipboard_exit(clipboard, args.clear_clipboard_on_exit, args.clipboard_ttl)?;

    if args.secure_exit {
        secret.zeroize();
    }

    Ok(())
}

fn run_encrypt_file(args: FileEncryptArgs) -> Result<()> {
    if !args.input.is_file() {
        bail!("input is not a file: {}", args.input.display());
    }

    let output = args
        .output
        .unwrap_or_else(|| default_encrypted_output(&args.input));
    if paths_same(&args.input, &output) {
        bail!("refusing to use the same path for input and encrypted output");
    }
    if output.exists() && !args.force {
        bail!(
            "output already exists: {}. Use --force to overwrite.",
            output.display()
        );
    }

    let mut plaintext = Zeroizing::new(
        fs::read(&args.input)
            .with_context(|| format!("failed to read {}", args.input.display()))?,
    );
    let password = prompt_new_file_password()?;
    // Do not store the plaintext source filename inside the encrypted envelope;
    // filenames can leak OPSEC-relevant metadata even when file contents are encrypted.
    let envelope = encrypt_envelope_with_kind(
        plaintext.as_slice(),
        password.as_bytes(),
        args.cipher,
        "file",
        None,
    )?;
    plaintext.zeroize();
    write_envelope(&output, &envelope)?;
    ui_success(&format!("Encrypted file written: {}", output.display()));
    Ok(())
}

fn run_decrypt_file(args: FileDecryptArgs) -> Result<()> {
    if paths_same(&args.input, &args.output) {
        bail!("refusing to use the same path for encrypted input and plaintext output");
    }
    if args.output.exists() && !args.force {
        bail!(
            "output already exists: {}. Use --force to overwrite.",
            args.output.display()
        );
    }

    let encrypted = fs::read(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let envelope: EncryptedEnvelope = serde_json::from_slice(&encrypted).with_context(|| {
        format!(
            "failed to parse encrypted envelope {}",
            args.input.display()
        )
    })?;

    let password = prompt_file_password("File encryption password: ")?;
    let mut plaintext = decrypt_envelope_to_bytes(&envelope, password.as_bytes())?;

    if let Some(parent) = args.output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create output directory {}", parent.display())
            })?;
        }
    }

    write_private_file(&args.output, plaintext.as_slice())?;
    plaintext.zeroize();
    ui_success(&format!(
        "Decrypted file written: {}",
        args.output.display()
    ));
    Ok(())
}

fn run_doctor(args: DoctorArgs) -> Result<()> {
    let exe = std::env::current_exe().context("failed to locate current executable")?;
    let bytes = fs::read(&exe).with_context(|| format!("failed to read {}", exe.display()))?;
    let digest = hex::encode(Sha256::digest(&bytes));

    println!("Executable: {}", exe.display());
    println!("SHA-256:    {digest}");

    if let Some(expected) = args.expected_sha256 {
        if digest.eq_ignore_ascii_case(expected.trim()) {
            ui_success("Tamper check: OK");
        } else {
            bail!("tamper check FAILED: hash does not match expected value");
        }
    }

    Ok(())
}

fn run_tpm_status() -> Result<()> {
    ui_section("TPM 2.0 status and integration notes");
    ui_info(
        "This build validates TPM availability but does not falsely claim TPM-backed encryption.",
    );

    if cfg!(target_os = "windows") {
        ui_info("Windows detected. Checking Get-Tpm through PowerShell...");
        let ps_command = r#"
$IsAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not (Get-Command Get-Tpm -ErrorAction SilentlyContinue)) {
    Write-Output 'TPM query unavailable: Get-Tpm command was not found on this system.'
    exit 0
}
if (-not $IsAdmin) {
    Write-Output 'TPM query requires Administrator privileges for reliable values. Re-run PowerShell as Administrator for TPM status.'
    exit 0
}
try {
    Get-Tpm | Select-Object TpmPresent,TpmReady,TpmEnabled,TpmActivated | Format-List
} catch {
    Write-Output ('TPM query failed: ' + $_.Exception.Message)
}
"#;
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", ps_command])
            .output()
            .context("failed to execute PowerShell TPM check")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stdout.trim().is_empty() {
            for line in stdout.lines() {
                if line.contains("True") {
                    ui_success(line.trim());
                } else if line.contains("requires Administrator")
                    || line.contains("unavailable")
                    || line.contains("failed")
                {
                    ui_warn(line.trim());
                } else if !line.trim().is_empty() {
                    ui_info(line.trim());
                }
            }
        }
        if !stderr.trim().is_empty() {
            ui_warn(stderr.trim());
        }
        ui_warn("TPM-backed file encryption is not enabled yet; real support requires Windows CNG/NCrypt key wrapping and test coverage.");
        ui_info("Current secure mode remains Argon2id + AEAD encryption with a random salt and nonce per file.");
        return Ok(());
    }

    if cfg!(target_os = "linux") {
        ui_info("Linux detected. Checking common TPM device paths...");
        let mut found = false;
        for candidate in ["/dev/tpmrm0", "/dev/tpm0"] {
            if Path::new(candidate).exists() {
                ui_success(&format!("{candidate}: present"));
                found = true;
            } else {
                ui_warn(&format!("{candidate}: not found"));
            }
        }
        if !found {
            ui_warn("No TPM device detected in this Linux/Kali environment.");
        }
        ui_info("Future Linux TPM support should use tpm2-tss/tss-esapi with PCR policy and dedicated tests.");
        return Ok(());
    }

    ui_warn("TPM status is not implemented for this operating system.");
    Ok(())
}

fn run_checklist_status() -> Result<()> {
    println!("{}", "Project OMEGA readiness checklist".with(Color::Cyan));
    println!("Platform: {}", std::env::consts::OS);
    println!();
    println!("{}", "PASS  Entropy: OS CSPRNG via rand_core::OsRng; optional hidden user entropy is mixed into a ChaCha20 CSPRNG seed.".with(Color::Green));
    println!("{}", "PASS  Uniqueness: local SHA-256 fingerprint history blocks local repeats while history file remains intact.".with(Color::Green));
    println!(
        "{}",
        "PASS  Password length: enforced 30 through 64 characters.".with(Color::Green)
    );
    println!("{}", "PASS  Encryption: authenticated encryption envelope using AES-256-GCM, XChaCha20-Poly1305, or AES-256-GCM-SIV.".with(Color::Green));
    println!(
        "{}",
        "PASS  KDF/salt: Argon2id with a fresh random 128-bit salt per encrypted envelope."
            .with(Color::Green)
    );
    println!("{}", "PASS  Memory hygiene: Rust memory safety plus zeroize on owned secret buffers where practical.".with(Color::Green));
    println!("{}", "PASS  UI: interactive colored menu and full manual command-line arguments are both supported.".with(Color::Green));
    println!("{}", "PASS  Terminal cleanup: best-effort cleanup helper exists for PowerShell/Bash/Zsh/Fish history files.".with(Color::Green));
    println!("{}", "PART  TPM 2.0: status detection is implemented; TPM-backed key wrapping is documented but must not be marked complete until platform API integration is tested on your Windows 11 host.".with(Color::Yellow));
    println!("{}", "PEND  Security audit: run cargo fmt, cargo check, cargo audit, cargo outdated, functional tests, and manual code review on Windows and Kali before final deployment.".with(Color::Yellow));
    println!();
    println!("{}", "No tool can guarantee global never-repeat without persistent shared state, and no history cleanup can erase screenshots, transcripts, clipboard managers, malware logs, or secrets pasted into chats.".with(Color::Yellow));
    Ok(())
}

fn run_clean_history(args: CleanHistoryArgs) -> Result<()> {
    clear_screen()?;
    ui_section("Best-effort terminal cleanup");
    ui_warn("This cannot erase screenshots, terminal emulator scrollback already saved by another app, PowerShell transcripts, malware logs, clipboard managers, or anything pasted into chats.");

    let candidates = history_candidates();
    if candidates.is_empty() {
        ui_info("No common shell history files found.");
        return Ok(());
    }

    if !args.yes {
        ui_info("History files that would be removed/truncated:");
        for path in candidates {
            println!("  {}", path.display());
        }
        ui_info("Run with --yes to actually remove these files.");
        return Ok(());
    }

    for path in candidates {
        match fs::remove_file(&path) {
            Ok(_) => ui_success(&format!("Removed: {}", path.display())),
            Err(e) => ui_warn(&format!("Could not remove {}: {e}", path.display())),
        }
    }

    if cfg!(target_os = "windows") {
        ui_info("Close and reopen PowerShell to fully reset in-memory PSReadLine history.");
    } else {
        ui_info("Run `history -c && history -w` in your current shell if your shell keeps history in memory.");
    }

    Ok(())
}

fn run_menu() -> Result<()> {
    loop {
        draw_banner()?;
        ui_section("Main command deck");
        ui_menu_item(
            "1",
            "Create secure passphrase",
            "CSPRNG + local no-repeat + vault",
        );
        ui_menu_item("2", "Encrypt / decrypt", "Vaults and .apv file envelopes");
        ui_menu_item(
            "3",
            "Tamper check",
            "Print or verify the executable SHA-256 hash",
        );
        ui_menu_item("4", "Terminal cleanup", "Best-effort shell history cleanup");
        ui_menu_item("5", "TPM 2.0 status", "Windows/Linux status + notes");
        ui_menu_item("6", "Environment checklist", "Readiness status");
        ui_menu_item("7", "Secure exit", "Clear screen and exit");
        println!();

        match prompt_line("Select option: ")?.trim() {
            "1" => handle_menu_screen("Passphrase workflow", menu_generate())?,
            "2" => handle_menu_screen("Encryption workflow", menu_crypto())?,
            "3" => {
                ui_info("Calculating executable SHA-256...");
                run_doctor(DoctorArgs {
                    expected_sha256: None,
                })?;
                pause_for_user()?;
            }
            "4" => {
                ui_warn("History cleanup is best-effort and cannot erase screenshots, transcripts, malware logs, or external clipboard managers.");
                let yes = prompt_yes_no("Actually remove common shell history files?", false)?;
                run_clean_history(CleanHistoryArgs { yes })?;
                pause_for_user()?;
            }
            "5" => {
                run_tpm_status()?;
                pause_for_user()?;
            }
            "6" => {
                run_checklist_status()?;
                pause_for_user()?;
            }
            "7" | "q" | "Q" => {
                clear_screen()?;
                ui_success("Protocol OMEGA secure exit complete.");
                break;
            }
            _ => {
                ui_error("Invalid option. Choose 1-7 or q.");
                pause_for_user()?;
            }
        }
    }
    Ok(())
}

fn draw_banner() -> Result<()> {
    clear_screen()?;
    let line = "=".repeat(76);
    println!("{}", line.as_str().with(Color::Blue));
    println!(
        "{}",
        "        P r o t o c o l    O M E G A    E n g a g e d".with(Color::Cyan)
    );
    println!("{}", line.as_str().with(Color::Blue));
    println!(
        "{}",
        "Defensive local secret generation • AEAD encryption • OPSEC hygiene".with(Color::Magenta)
    );
    println!(
        "{}",
        "[*] Status markers: [*] info  [+] pass  [!] warning  [-] error".with(Color::DarkGrey)
    );
    Ok(())
}

fn platform_secret_path_example() -> &'static str {
    if cfg!(windows) {
        r".\secret.apv"
    } else {
        "./secret.apv"
    }
}

fn platform_file_path_example() -> &'static str {
    if cfg!(windows) {
        r"C:\Users\<User>\Desktop\example.txt or .\example.txt"
    } else {
        "/home/<user>/Desktop/example.txt or ./example.txt"
    }
}

fn is_back_input(input: &str) -> bool {
    matches!(
        input.trim().to_lowercase().as_str(),
        "b" | "back" | ":back" | "cancel" | "q" | "quit"
    )
}

fn back_error() -> anyhow::Error {
    anyhow!("operation cancelled; returning to previous menu")
}

fn handle_menu_screen(action: &str, result: Result<()>) -> Result<()> {
    if let Err(error) = result {
        ui_warn(&format!("{action}: {error}"));
        pause_for_user()?;
    }
    Ok(())
}

fn menu_generate() -> Result<()> {
    draw_banner()?;
    ui_section("Secure passphrase generation");
    ui_info("Generation uses OS randomness mixed into ChaCha20 CSPRNG state.");
    ui_info(
        "Local SHA-256 fingerprint history blocks repeats while the history file remains intact.",
    );
    ui_info("Type `back` at most prompts to return to the previous menu without making changes.");

    let length = prompt_length("Length 30-64", DEFAULT_LENGTH)?;
    let charset = prompt_charset()?;
    let extra_entropy = prompt_yes_no("Add hidden user entropy?", true)?;
    let hide = prompt_yes_no("Hide generated secret from terminal output?", true)?;
    let copy = prompt_yes_no("Copy generated secret to clipboard?", true)?;
    let clipboard_ttl = if copy {
        Some(prompt_u64("Clipboard TTL seconds", 20)?)
    } else {
        None
    };
    let save = if prompt_yes_no("Save encrypted .apv vault file?", false)? {
        Some(prompt_output_path(&format!(
            "Output file path, example {}: ",
            platform_secret_path_example()
        ))?)
    } else {
        None
    };
    let cipher = if save.is_some() {
        prompt_cipher()?
    } else {
        CipherChoice::Aes256Gcm
    };

    ui_info("Executing passphrase generation request...");
    let args = GenArgs {
        length,
        charset,
        copy,
        extra_entropy,
        save,
        cipher,
        hide,
        no_history: false,
        history_file: None,
        secure_exit: true,
        clear_clipboard_on_exit: false,
        clipboard_ttl,
    };
    handle_menu_result("Passphrase generation", run_gen(args))?;
    Ok(())
}

fn handle_menu_result(action: &str, result: Result<()>) -> Result<()> {
    match result {
        Ok(()) => {
            pause_for_user()?;
            Ok(())
        }
        Err(error) => {
            ui_error(&format!("{action} failed: {error}"));
            pause_for_user()?;
            Ok(())
        }
    }
}

fn menu_crypto() -> Result<()> {
    loop {
        draw_banner()?;
        ui_section("Encryption and decryption deck");
        ui_info(&format!(
            "Paths may be current-directory paths or full paths, e.g. {}",
            platform_file_path_example()
        ));
        ui_info("Type `back` at a path prompt to return to the previous menu.");
        ui_menu_item(
            "1",
            "Decrypt passphrase vault",
            "Hidden output or clipboard",
        );
        ui_menu_item(
            "2",
            "Encrypt arbitrary file",
            "Create authenticated .apv envelope",
        );
        ui_menu_item("3", "Decrypt arbitrary file", "Recover plaintext file");
        ui_menu_item("4", "TPM 2.0 file mode notes", "Design status, not enabled");
        ui_menu_item("5", "Back", "Return to main menu");
        println!();

        match prompt_line("Select option: ")?.trim() {
            "1" => {
                let input = prompt_existing_path("Vault input path: ")?;
                let hide = prompt_yes_no("Hide decrypted secret from terminal output?", true)?;
                let copy = prompt_yes_no("Copy decrypted secret to clipboard?", true)?;
                let clipboard_ttl = if copy {
                    Some(prompt_u64("Clipboard TTL seconds", 20)?)
                } else {
                    None
                };
                ui_info("Decrypting passphrase vault...");
                handle_menu_result(
                    "Passphrase vault decryption",
                    run_decrypt(DecryptArgs {
                        input,
                        copy,
                        hide,
                        clear_clipboard_on_exit: false,
                        clipboard_ttl,
                        secure_exit: true,
                    }),
                )?;
            }
            "2" => {
                let input = prompt_existing_path("File to encrypt: ")?;
                let default_output = default_encrypted_output(&input);
                let output_raw = prompt_line(&format!(
                    "Output .apv path [{}]: ",
                    default_output.display()
                ))?;
                let output_trimmed = output_raw.trim();
                if is_back_input(output_trimmed) {
                    return Err(back_error());
                }
                let output = if output_trimmed.is_empty() {
                    default_output
                } else {
                    validate_output_path(parse_user_path(output_trimmed))?
                };
                let cipher = prompt_cipher()?;
                let force = prompt_yes_no("Overwrite output if it exists?", false)?;
                ui_info("Encrypting file into authenticated .apv envelope...");
                handle_menu_result(
                    "File encryption",
                    run_encrypt_file(FileEncryptArgs {
                        input,
                        output: Some(output),
                        cipher,
                        force,
                    }),
                )?;
            }
            "3" => {
                let input = prompt_existing_path("Encrypted .apv input path: ")?;
                let output = prompt_output_path("Output plaintext file path: ")?;
                let force = prompt_yes_no("Overwrite output if it exists?", false)?;
                ui_info("Decrypting authenticated .apv envelope...");
                handle_menu_result(
                    "File decryption",
                    run_decrypt_file(FileDecryptArgs {
                        input,
                        output,
                        force,
                    }),
                )?;
            }
            "4" => {
                menu_tpm_file_mode_notes()?;
                pause_for_user()?;
            }
            "5" | "b" | "B" => break,
            _ => {
                ui_error("Invalid option. Choose 1-5 or b.");
                pause_for_user()?;
            }
        }
    }
    Ok(())
}

fn menu_tpm_file_mode_notes() -> Result<()> {
    ui_section("Windows TPM 2.0 file encryption mode notes");
    run_tpm_status()?;
    ui_warn(
        "TPM-backed file encryption/decryption is intentionally not marked complete in this build.",
    );
    ui_info("Reason: real TPM-backed file encryption must wrap or seal file keys through Windows CNG/NCrypt with Microsoft Platform Crypto Provider and then be tested on Windows 11 hardware.");
    ui_info("Current production candidate mode remains password-derived Argon2id + AEAD encryption with random 128-bit salts and random nonces.");
    ui_info("Planned TPM v1.0 gate: add a Windows-only file-key wrapping layer, metadata version bump, migration tests, and recovery documentation.");
    Ok(())
}

fn parse_length(value: &str) -> std::result::Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| "length must be a number".to_string())?;

    if (30..=64).contains(&parsed) {
        Ok(parsed)
    } else {
        Err("length must be between 30 and 64 characters".to_string())
    }
}

fn prompt_extra_entropy() -> Result<Zeroizing<String>> {
    let entropy = rpassword::prompt_password(
        "Optional entropy input, hidden. Type random words/keystrokes and press Enter: ",
    )?;
    Ok(Zeroizing::new(entropy))
}

fn prompt_file_password(prompt: &str) -> Result<Zeroizing<String>> {
    let password = rpassword::prompt_password(prompt)?;
    if password.is_empty() {
        bail!("empty encryption password refused");
    }
    Ok(Zeroizing::new(password))
}

fn prompt_new_file_password() -> Result<Zeroizing<String>> {
    let password_one = prompt_file_password("Create file encryption password: ")?;
    let password_two = prompt_file_password("Confirm file encryption password: ")?;

    if password_one.as_str() != password_two.as_str() {
        bail!("file encryption passwords did not match");
    }
    if password_one.chars().count() < MIN_FILE_PASSWORD_LEN {
        bail!("file encryption password must be at least {MIN_FILE_PASSWORD_LEN} characters");
    }

    Ok(password_one)
}

fn generate_unique_secret(
    length: usize,
    charset: &[u8],
    extra_entropy: Option<&Zeroizing<String>>,
    history_path: Option<&Path>,
) -> Result<Zeroizing<String>> {
    for _ in 0..128 {
        let secret = generate_secret(length, charset, extra_entropy)?;

        if let Some(path) = history_path {
            let fingerprint = fingerprint_secret(secret.as_str());
            if history_contains(path, &fingerprint)? {
                continue;
            }
            append_history(path, &fingerprint)?;
        }

        return Ok(secret);
    }

    bail!("failed to generate a locally unique passphrase after 128 attempts")
}

fn generate_secret(
    length: usize,
    charset: &[u8],
    extra_entropy: Option<&Zeroizing<String>>,
) -> Result<Zeroizing<String>> {
    if charset.is_empty() || charset.len() > 256 {
        bail!("invalid charset length");
    }

    let mut rng = build_rng(extra_entropy)?;
    let zone = 256usize - (256usize % charset.len());
    let mut out = Vec::with_capacity(length);

    while out.len() < length {
        let mut b = [0u8; 1];
        rng.fill_bytes(&mut b);
        let value = b[0] as usize;
        if value < zone {
            out.push(charset[value % charset.len()]);
        }
        b.zeroize();
    }

    let secret = String::from_utf8(out).context("generated bytes were not valid UTF-8")?;
    Ok(Zeroizing::new(secret))
}

fn build_rng(extra_entropy: Option<&Zeroizing<String>>) -> Result<ChaCha20Rng> {
    let mut os_seed = Zeroizing::new([0u8; 64]);
    OsRng.fill_bytes(&mut *os_seed);

    let mut hasher = Sha512::new();
    hasher.update(RNG_DOMAIN);
    hasher.update(&*os_seed);

    if let Some(entropy) = extra_entropy {
        hasher.update(entropy.as_bytes());
    }

    let digest = hasher.finalize();
    let mut seed = Zeroizing::new([0u8; 32]);
    seed.copy_from_slice(&digest[..32]);

    Ok(ChaCha20Rng::from_seed(*seed))
}

fn fingerprint_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(HISTORY_DOMAIN);
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

fn resolve_history_path(custom: Option<&PathBuf>) -> Result<PathBuf> {
    if let Some(path) = custom {
        return Ok(path.clone());
    }

    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        return Ok(PathBuf::from(user_profile).join(".aegisphrase_history"));
    }

    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(".aegisphrase_history"));
    }

    Ok(PathBuf::from(".aegisphrase_history"))
}

fn history_contains(path: &Path, fingerprint: &str) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read history file {}", path.display()))?;
    Ok(content.lines().any(|line| line.trim() == fingerprint))
}

fn append_history(path: &Path, fingerprint: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create history directory {}", parent.display())
            })?;
        }
    }

    append_private_line(path, fingerprint)?;
    Ok(())
}

fn save_encrypted_secret(path: &Path, secret: &str, cipher: CipherChoice) -> Result<()> {
    if path.exists() {
        bail!(
            "output already exists: {}. Choose a new vault path to avoid accidental overwrite.",
            path.display()
        );
    }
    let password = prompt_new_file_password()?;
    let plaintext = Zeroizing::new(format!("{secret}\n"));
    let envelope = encrypt_envelope_with_kind(
        plaintext.as_bytes(),
        password.as_bytes(),
        cipher,
        "passphrase",
        None,
    )?;
    write_envelope(path, &envelope)?;
    Ok(())
}

fn write_envelope(path: &Path, envelope: &EncryptedEnvelope) -> Result<()> {
    let serialized =
        serde_json::to_vec_pretty(envelope).context("failed to serialize encrypted envelope")?;

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create output directory {}", parent.display())
            })?;
        }
    }

    write_private_file(path, &serialized)?;
    Ok(())
}

fn encrypt_envelope_with_kind(
    plaintext: &[u8],
    password: &[u8],
    cipher: CipherChoice,
    kind: &str,
    source_name: Option<String>,
) -> Result<EncryptedEnvelope> {
    let kdf = KdfMetadata {
        name: "argon2id".to_string(),
        memory_kib: KDF_MEMORY_KIB,
        iterations: KDF_ITERATIONS,
        lanes: KDF_LANES,
        output_len: KDF_OUTPUT_LEN,
    };

    let mut salt = Zeroizing::new(vec![0u8; SALT_LEN]);
    let mut nonce = Zeroizing::new(vec![0u8; cipher.nonce_len()]);
    OsRng.fill_bytes(salt.as_mut_slice());
    OsRng.fill_bytes(nonce.as_mut_slice());

    let key = derive_key(password, salt.as_slice(), &kdf)?;
    let ciphertext = encrypt_bytes(cipher, &*key, nonce.as_slice(), plaintext)?;

    Ok(EncryptedEnvelope {
        format: FILE_FORMAT.to_string(),
        version: 2,
        cipher: cipher.wire_name().to_string(),
        kdf,
        salt_b64: B64.encode(salt.as_slice()),
        nonce_b64: B64.encode(nonce.as_slice()),
        ciphertext_b64: B64.encode(ciphertext),
        created_utc: chrono::Utc::now().to_rfc3339(),
        kind: Some(kind.to_string()),
        source_name,
    })
}

fn decrypt_envelope_to_bytes(
    envelope: &EncryptedEnvelope,
    password: &[u8],
) -> Result<Zeroizing<Vec<u8>>> {
    if !((envelope.format == FILE_FORMAT && envelope.version == 2)
        || (envelope.format == LEGACY_FILE_FORMAT && envelope.version == 1))
    {
        bail!("unsupported encrypted file format or version");
    }

    let cipher = CipherChoice::from_wire_name(envelope.cipher.as_str())?;
    let salt = B64
        .decode(envelope.salt_b64.as_bytes())
        .context("invalid salt base64")?;
    let nonce = B64
        .decode(envelope.nonce_b64.as_bytes())
        .context("invalid nonce base64")?;
    let ciphertext = B64
        .decode(envelope.ciphertext_b64.as_bytes())
        .context("invalid ciphertext base64")?;

    validate_envelope_parts(cipher, &salt, &nonce, &ciphertext, &envelope.kdf)?;
    let key = derive_key(password, &salt, &envelope.kdf)?;
    let plaintext = decrypt_bytes(cipher, &*key, &nonce, &ciphertext)?;
    Ok(Zeroizing::new(plaintext))
}

fn validate_envelope_parts(
    cipher: CipherChoice,
    salt: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    kdf: &KdfMetadata,
) -> Result<()> {
    validate_kdf_metadata(kdf)?;
    if salt.len() != SALT_LEN {
        bail!("invalid encrypted file: unexpected salt length");
    }
    if nonce.len() != cipher.nonce_len() {
        bail!("invalid encrypted file: unexpected nonce length");
    }
    if ciphertext.is_empty() {
        bail!("invalid encrypted file: empty ciphertext");
    }
    Ok(())
}

fn validate_kdf_metadata(kdf: &KdfMetadata) -> Result<()> {
    if kdf.name != "argon2id"
        || kdf.memory_kib != KDF_MEMORY_KIB
        || kdf.iterations != KDF_ITERATIONS
        || kdf.lanes != KDF_LANES
        || kdf.output_len != KDF_OUTPUT_LEN
    {
        bail!("unsupported or unsafe KDF settings in encrypted file");
    }
    Ok(())
}

fn derive_key(password: &[u8], salt: &[u8], kdf: &KdfMetadata) -> Result<Zeroizing<[u8; 32]>> {
    validate_kdf_metadata(kdf)?;

    let params = argon2::Params::new(
        kdf.memory_kib,
        kdf.iterations,
        kdf.lanes,
        Some(kdf.output_len),
    )
    .map_err(|e| anyhow!("invalid Argon2id parameters: {e}"))?;
    let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut key = Zeroizing::new([0u8; 32]);
    argon2
        .hash_password_into(password, salt, &mut *key)
        .map_err(|e| anyhow!("Argon2id key derivation failed: {e}"))?;

    Ok(key)
}

fn encrypt_bytes(
    cipher: CipherChoice,
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>> {
    match cipher {
        CipherChoice::Aes256Gcm => {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|_| anyhow!("invalid AES-256-GCM key length"))?;
            cipher
                .encrypt(AesNonce::from_slice(nonce), plaintext)
                .map_err(|_| anyhow!("AES-256-GCM encryption failed"))
        }
        CipherChoice::XChaCha20Poly1305 => {
            let cipher = XChaCha20Poly1305::new_from_slice(key)
                .map_err(|_| anyhow!("invalid XChaCha20-Poly1305 key length"))?;
            cipher
                .encrypt(XNonce::from_slice(nonce), plaintext)
                .map_err(|_| anyhow!("XChaCha20-Poly1305 encryption failed"))
        }
        CipherChoice::Aes256GcmSiv => {
            let cipher = Aes256GcmSiv::new_from_slice(key)
                .map_err(|_| anyhow!("invalid AES-256-GCM-SIV key length"))?;
            cipher
                .encrypt(AesSivNonce::from_slice(nonce), plaintext)
                .map_err(|_| anyhow!("AES-256-GCM-SIV encryption failed"))
        }
    }
}

fn decrypt_bytes(
    cipher: CipherChoice,
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    match cipher {
        CipherChoice::Aes256Gcm => {
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|_| anyhow!("invalid AES-256-GCM key length"))?;
            cipher
                .decrypt(AesNonce::from_slice(nonce), ciphertext)
                .map_err(|_| {
                    anyhow!("AES-256-GCM decryption failed: wrong password or corrupt file")
                })
        }
        CipherChoice::XChaCha20Poly1305 => {
            let cipher = XChaCha20Poly1305::new_from_slice(key)
                .map_err(|_| anyhow!("invalid XChaCha20-Poly1305 key length"))?;
            cipher
                .decrypt(XNonce::from_slice(nonce), ciphertext)
                .map_err(|_| {
                    anyhow!("XChaCha20-Poly1305 decryption failed: wrong password or corrupt file")
                })
        }
        CipherChoice::Aes256GcmSiv => {
            let cipher = Aes256GcmSiv::new_from_slice(key)
                .map_err(|_| anyhow!("invalid AES-256-GCM-SIV key length"))?;
            cipher
                .decrypt(AesSivNonce::from_slice(nonce), ciphertext)
                .map_err(|_| {
                    anyhow!("AES-256-GCM-SIV decryption failed: wrong password or corrupt file")
                })
        }
    }
}

fn copy_to_clipboard(secret: &str) -> Result<Clipboard> {
    let mut clipboard = Clipboard::new().context("failed to open system clipboard")?;
    clipboard
        .set_text(secret.to_string())
        .context("failed to write to system clipboard")?;
    Ok(clipboard)
}

fn clear_clipboard(clipboard: &mut Clipboard) -> Result<()> {
    clipboard
        .clear()
        .or_else(|_| clipboard.set_text(String::new()))
        .context("failed to clear system clipboard")?;
    Ok(())
}

fn handle_clipboard_exit(
    mut clipboard: Option<Clipboard>,
    clear_on_exit: bool,
    ttl: Option<u64>,
) -> Result<()> {
    let Some(ref mut clipboard) = clipboard else {
        return Ok(());
    };

    if let Some(seconds) = ttl {
        ui_info(&format!(
            "Clipboard will be cleared in {seconds} seconds. Keep this terminal open."
        ));
        thread::sleep(Duration::from_secs(seconds));
        clear_clipboard(clipboard)?;
        ui_success("Clipboard cleared.");
        return Ok(());
    }

    if clear_on_exit {
        clear_clipboard(clipboard)?;
        ui_success("Clipboard cleared.");
    }

    Ok(())
}

fn paths_same(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    if left.exists() && right.exists() {
        if let (Ok(left), Ok(right)) = (fs::canonicalize(left), fs::canonicalize(right)) {
            return left == right;
        }
    }
    false
}

#[cfg(unix)]
fn write_private_file(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .with_context(|| format!("failed to open {} for private write", path.display()))?;
    file.write_all(bytes)
        .with_context(|| format!("failed to write {}", path.display()))?;
    let mut permissions = file
        .metadata()
        .with_context(|| format!("failed to read permissions for {}", path.display()))?
        .permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)
        .with_context(|| format!("failed to set private permissions on {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn write_private_file(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(unix)]
fn append_private_line(path: &Path, line: &str) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(path)
        .with_context(|| format!("failed to open history file {}", path.display()))?;
    writeln!(file, "{line}").context("failed to append history fingerprint")?;
    let mut permissions = file
        .metadata()
        .with_context(|| format!("failed to read permissions for {}", path.display()))?
        .permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)
        .with_context(|| format!("failed to set private permissions on {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn append_private_line(path: &Path, line: &str) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open history file {}", path.display()))?;
    writeln!(file, "{line}").context("failed to append history fingerprint")?;
    Ok(())
}

fn default_encrypted_output(input: &Path) -> PathBuf {
    let mut os = input.as_os_str().to_os_string();
    os.push(".apv");
    PathBuf::from(os)
}

fn prompt_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end_matches(['\r', '\n']).to_string())
}

fn parse_user_path(input: &str) -> PathBuf {
    let trimmed = input.trim();
    #[cfg(unix)]
    {
        if trimmed.contains('\\') {
            ui_warn(
                "Windows-style backslashes detected on Linux/Kali; converting to forward slashes.",
            );
            return PathBuf::from(trimmed.replace('\\', "/"));
        }
    }
    PathBuf::from(trimmed)
}

fn validate_output_path(path: PathBuf) -> Result<PathBuf> {
    if path.as_os_str().is_empty() {
        bail!("path cannot be empty");
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            bail!("parent directory does not exist: {}", parent.display());
        }
    }

    Ok(path)
}

fn prompt_output_path(prompt: &str) -> Result<PathBuf> {
    loop {
        let input = prompt_line(prompt)?;
        let trimmed = input.trim();
        if is_back_input(trimmed) {
            return Err(back_error());
        }
        if trimmed.is_empty() {
            ui_warn("Path cannot be empty. Enter a file path or type `back` to return.");
            continue;
        }

        match validate_output_path(parse_user_path(trimmed)) {
            Ok(path) => return Ok(path),
            Err(error) => ui_warn(&format!("{error}")),
        }
    }
}

fn prompt_existing_path(prompt: &str) -> Result<PathBuf> {
    loop {
        let path = prompt_output_path(prompt)?;
        if path.exists() {
            return Ok(path);
        }
        ui_warn(&format!("Path does not exist: {}", path.display()));
    }
}

fn prompt_yes_no(prompt: &str, default: bool) -> Result<bool> {
    let suffix = if default { "[Y/n]" } else { "[y/N]" };
    let input = prompt_line(&format!("{prompt} {suffix}: "))?;
    let trimmed_raw = input.trim();
    if is_back_input(trimmed_raw) {
        return Err(back_error());
    }
    let trimmed = trimmed_raw.to_lowercase();
    if trimmed.is_empty() {
        return Ok(default);
    }
    Ok(matches!(trimmed.as_str(), "y" | "yes"))
}

fn prompt_length(prompt: &str, default: usize) -> Result<usize> {
    loop {
        let input = prompt_line(&format!("{prompt} [{default}]: "))?;
        let trimmed = input.trim();
        if is_back_input(trimmed) {
            return Err(back_error());
        }
        if trimmed.is_empty() {
            return Ok(default);
        }
        match parse_length(trimmed) {
            Ok(value) => return Ok(value),
            Err(e) => ui_warn(&format!("{e}")),
        }
    }
}

fn prompt_u64(prompt: &str, default: u64) -> Result<u64> {
    loop {
        let input = prompt_line(&format!("{prompt} [{default}]: "))?;
        let trimmed = input.trim();
        if is_back_input(trimmed) {
            return Err(back_error());
        }
        if trimmed.is_empty() {
            return Ok(default);
        }
        match trimmed.parse::<u64>() {
            Ok(value) => return Ok(value),
            Err(_) => ui_warn("Enter a number."),
        }
    }
}

fn prompt_charset() -> Result<CharsetChoice> {
    ui_section("Charset options");
    println!("{}", "[1] safe       recommended".with(Color::Green));
    println!(
        "{}",
        "[2] full       printable ASCII symbols".with(Color::Green)
    );
    println!("{}", "[3] base64url  most compatible".with(Color::Green));
    println!(
        "{}",
        "[4] hex        compatible, lower entropy per char".with(Color::Yellow)
    );
    loop {
        let input = prompt_line("Choose charset [1]: ")?;
        let trimmed = input.trim();
        if is_back_input(trimmed) {
            return Err(back_error());
        }
        match trimmed {
            "" | "1" => return Ok(CharsetChoice::Safe),
            "2" => return Ok(CharsetChoice::Full),
            "3" => return Ok(CharsetChoice::Base64Url),
            "4" => return Ok(CharsetChoice::Hex),
            _ => ui_error("Invalid charset."),
        }
    }
}

fn prompt_cipher() -> Result<CipherChoice> {
    ui_section("Cipher options");
    println!(
        "{}",
        "[1] aes256-gcm             default, NIST GCM AEAD mode".with(Color::Green)
    );
    println!(
        "{}",
        "[2] xchacha20-poly1305     large nonce, excellent general choice".with(Color::Green)
    );
    println!(
        "{}",
        "[3] aes256-gcm-siv         nonce-misuse resistant".with(Color::Green)
    );
    loop {
        let input = prompt_line("Choose cipher [1]: ")?;
        let trimmed = input.trim();
        if is_back_input(trimmed) {
            return Err(back_error());
        }
        match trimmed {
            "" | "1" => return Ok(CipherChoice::Aes256Gcm),
            "2" => return Ok(CipherChoice::XChaCha20Poly1305),
            "3" => return Ok(CipherChoice::Aes256GcmSiv),
            _ => ui_error("Invalid cipher."),
        }
    }
}

fn history_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(
            PathBuf::from(appdata)
                .join("Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt"),
        );
    }

    if let Ok(home) = std::env::var("HOME") {
        let home = PathBuf::from(home);
        paths.push(home.join(".bash_history"));
        paths.push(home.join(".zsh_history"));
        paths.push(home.join(".fish_history"));
        paths.push(home.join(".python_history"));
    }

    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let home = PathBuf::from(user_profile);
        paths.push(home.join(".bash_history"));
        paths.push(home.join(".zsh_history"));
        paths.push(home.join(".python_history"));
    }

    paths.into_iter().filter(|p| p.exists()).collect()
}

fn pause_for_user() -> Result<()> {
    let mut input = prompt_line("Press Enter to continue...")?;
    input.zeroize();
    Ok(())
}
