# Source Map

Use primary and official sources first.

## Core References

- Rust Documentation: https://doc.rust-lang.org/
- The Rust Book: https://doc.rust-lang.org/book/
- Serde: https://serde.rs/
- Microsoft Windows Event Tracing documentation: https://learn.microsoft.com/windows/win32/etw/event-tracing-portal
- Linux procfs documentation: https://docs.kernel.org/filesystems/proc.html
- eBPF documentation: https://ebpf.io/
- Elastic Common Schema: https://www.elastic.co/guide/en/ecs/current/index.html
- Open Cybersecurity Schema Framework: https://schema.ocsf.io/

## Source Rules

- Start with official docs.
- Record date accessed.
- Do not implement privileged telemetry until the userland MVP is stable.
- Keep examples local, safe, and reversible.