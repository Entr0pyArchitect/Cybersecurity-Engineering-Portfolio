# Assignment 2 – Plan, Environment, and MVP
## Track 5: Password Policy & Cracking Demo (Safe / Offline)

### Student
Entr0pyArchitect

### Course
Cybersecurity Capstone Capstone Seminar

---

## Assignment Context Clarification

This project is being completed under Track 5: Password Policy & Cracking Demo (Safe / Offline), as approved in the project proposal.

While general Assignment 2 instructions may reference repository secret scanning workflows (e.g., Gitleaks or pre-commit), this project follows the Track 5 pathway, which focuses on controlled offline password testing using synthetic data.

Accordingly, the environment, tools, and MVP demonstration presented in this submission are aligned specifically with Track 5 requirements, including:

- local password dataset generation
- controlled hash creation
- offline cracking baseline
- measurable results for later comparison

This ensures the assignment remains fully consistent with the approved project scope and course guidelines.

---

## 1. Project Overview

This project evaluates how password policy strength affects resistance to offline password cracking in a controlled lab environment. The core idea is to compare a weak password policy condition against a stronger password policy condition and measure the practical difference in crackability.

The project is intentionally narrow in scope. It focuses on one cybersecurity problem: whether stronger password practices measurably reduce the success of offline password cracking when compared to weaker password practices. The work is being performed on a personally owned system using artificial data only.

---

## 2. Problem Statement

Weak passwords remain one of the most common causes of credential compromise. Even when no live attack is involved, exposed password hashes can be subjected to offline cracking attempts. If passwords are short, common, or predictable, they are often significantly easier to recover. This project demonstrates that problem in a controlled and safe environment by comparing how weak and stronger password policies perform under the same local testing conditions.

---

## 3. Scope

This project is limited to a single personal lab system and an offline workflow using fake data only. No real user accounts, real passwords, production systems, third-party assets, or network attacks are involved.

The scope includes:

- generating artificial test users and passwords
- creating a weak password dataset
- later creating a stronger password dataset
- hashing the passwords locally
- running controlled offline cracking attempts
- recording measurable results for comparison

The scope does not include:

- attacking live targets
- using real credentials
- phishing or credential stuffing
- network exploitation
- persistence or unauthorized access

---

## 4. Environment

The Assignment 2 environment consists of a Kali Linux system running on a personally owned machine.

Tools validated:

- Python 3.13.9
- John the Ripper (jumbo build)
- Standard Linux command-line utilities

This environment supports the required Assignment 2 milestone of plan, setup, and MVP baseline execution.

---

## 5. Methodology

### Phase 1: Environment validation
Verify Python and John the Ripper are installed and working correctly.

### Phase 2: Synthetic data generation
Generate artificial users and weak passwords using a Python script.

### Phase 3: Hash generation
Convert passwords into SHA-256 hashes for controlled testing.

### Phase 4: Baseline cracking test
Use John the Ripper with a controlled wordlist to simulate an offline attack.

### Phase 5: Future comparison
Introduce stronger password policies and compare results.

---

## 6. Success Criteria

Assignment 2 is successful if:

- the environment is operational
- tools are installed and functioning
- synthetic data is generated
- hashes are created
- baseline cracking is executed
- results are recorded

Measurable improvement will be defined using:

- percentage of hashes cracked
- time required to crack
- attack effort required

---

## 7. MVP Definition

The MVP is a working baseline demonstration showing:

- synthetic data generation
- hash creation
- successful cracking execution
- captured results

---

## 8. Evidence Collected

- Python installation proof
- John the Ripper installation proof
- project directory structure
- generated dataset
- hash file
- cracking output
- results summary

---

## 9. Risks

- Tool misconfiguration: Incorrect John format or setup could produce invalid results
- Dataset bias: Weak password dataset may be overly predictable and not reflect real-world complexity

---

## 10. Ethics and Legal Boundaries

All testing is conducted on a personally owned system using artificial data only. No real systems, credentials, or unauthorized targets are involved.

---

## 11. Assignment 2 MVP Baseline Result

The Assignment 2 MVP baseline was successfully completed. Synthetic weak-password data was generated, hashed, and tested using John the Ripper.

All 12 password entries were successfully recovered, resulting in a 100% crack rate.

This baseline demonstrates that weak password policies are highly vulnerable under offline conditions and establishes a measurable starting point for comparison against stronger password policies.

The environment, toolchain, and workflow have been validated successfully.
