# Assignment 4 Explanation

For Assignment 4, I added a password-policy enforcement control to my Track 5 capstone project. The purpose of this improvement was to prevent weak passwords from being introduced into the stronger password dataset used in the project.

I created a validation script that checks each password in the strong dataset against the project’s policy requirements. The policy requires a minimum length of 14 characters, the presence of uppercase and lowercase letters, at least one number, at least one symbol, and rejection of weak denylisted passwords. I also created a shell wrapper to run the check as a simple enforcement step before accepting changes.

To demonstrate the control, I intentionally added a weak password to the strong dataset and ran the validator. The script correctly failed and reported that the password violated the policy. After restoring the valid dataset, the script passed successfully.

This improvement adds a preventive security control to the project. Instead of only showing that weak passwords are crackable, the project now also demonstrates how policy enforcement can block weak passwords before they are used.

## Evidence Locations

- Blocked policy screenshot: `evidence/screenshots/assignment4_blocked_policy_check.png`
- Clean policy screenshot: `evidence/screenshots/assignment4_Clean_Policy_Check.png`
- Validation script: `scripts/validate_password_policy.py`
- Policy wrapper script: `scripts/run_policy_check.sh`
