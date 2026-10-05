# Cybersecurity Capstone — Track 5

<p align="center">
  <img alt="Status: Completed Capstone" src="https://badges.ws/badge/Status-Completed%20Capstone-15803d?style=for-the-badge">
  <img alt="Focus: Password Policy" src="https://badges.ws/badge/Focus-Password%20Policy-0b7285?style=for-the-badge">
  <img alt="Data: Synthetic Only" src="https://badges.ws/badge/Data-Synthetic%20Only-6f42c1?style=for-the-badge">
</p>

A controlled applied-learning project that compares weak and stronger password-policy outcomes using **synthetic accounts, locally generated hashes, offline cracking, and documented validation results**.

The public version is intentionally sanitized. Academic submission binaries, video exports, screenshots, identity-bearing files, and environment-specific tree captures are kept local.

## Objective

Demonstrate, in a controlled lab, how password quality changes resistance to a fixed offline cracking workflow.

The project is designed around a simple comparison:

1. generate synthetic weak credentials;
2. hash them locally;
3. run a controlled offline cracking baseline;
4. generate a stronger synthetic policy/dataset;
5. repeat the same style of validation;
6. compare the measured outcomes.

This project does **not** use real user credentials, live authentication systems, phishing, credential stuffing, network exploitation, persistence, or unauthorized targets.

## Public repository layout

```text
Cybersecurity Capstone - Track 5/
├── README.md
├── ai-usage-appendix.md
├── data/
│   ├── strong_hashes.txt
│   ├── strong_passwords.txt
│   ├── strong_users.csv
│   ├── users.csv
│   ├── weak_hashes.txt
│   └── weak_passwords.txt
├── docs/
│   ├── assignment4_explanation.md
│   └── plan-v1.md
├── evidence/
│   ├── baseline/
│   │   └── john_run_output.txt
│   ├── env-proof/
│   │   ├── john_build_info.txt
│   │   └── python_version.txt
│   └── strong/
│       └── john_strong_run_output.txt
├── results/
│   ├── baseline_results.txt
│   └── strong_results.txt
└── scripts/
    ├── generate_strong_data.py
    ├── generate_test_data.py
    ├── run_baseline.sh
    ├── run_policy_check.sh
    └── validate_password_policy.py
```

## Synthetic data notice

The usernames and passwords under `data/` are intentionally artificial test data created for this project. They are **not** real credentials.

Weak-password examples are deliberately predictable so the baseline can demonstrate policy risk. Stronger examples are also synthetic and exist only for the controlled comparison.

## Validation workflow

### Python / shell syntax

```bash
python3 -m py_compile scripts/*.py

for f in scripts/*.sh; do
    bash -n "$f"
done
```

### Controlled lab workflow

The scripts are intended to be run from this project directory with the required local tooling installed.

```bash
python3 scripts/generate_test_data.py
bash scripts/run_baseline.sh

python3 scripts/generate_strong_data.py
bash scripts/run_policy_check.sh
```

Review the recorded output under `evidence/` and `results/` rather than treating script execution alone as proof.

## What this project demonstrates

- controlled dataset generation;
- basic password-policy reasoning;
- local hash generation;
- repeatable offline validation;
- comparison of weak and stronger password choices;
- separation of raw evidence from summarized results;
- ethical scoping of password-security experiments.

## Limitations

This is an educational lab, not a population-scale password study. Results depend on the chosen synthetic datasets, cracking configuration, candidate wordlists, hashing choices, and available compute.

The project therefore supports a narrow claim: **under the documented controlled conditions, stronger synthetic password choices were more resistant to the same style of offline guessing than the deliberately weak baseline.**

## Safety and ethics

All cracking activity is limited to locally generated synthetic hashes. Do not substitute real credential dumps, third-party password databases, or credentials obtained without explicit authorization.

[← Back to Blue Team](../../README.md)
