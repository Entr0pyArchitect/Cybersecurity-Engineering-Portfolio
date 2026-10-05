# FortKnox Threat Model

## Assets

- Message plaintext
- Private identity keys
- Device keys
- Group/session secrets
- Contact graph
- Attachment contents
- Account metadata

## Server Trust Model

The server is untrusted for message confidentiality. It may relay, queue, and synchronize encrypted messages, but it must never receive plaintext or private keys.

## Primary Threats

- Network surveillance
- Server compromise
- Stolen client device
- Malicious account takeover
- Metadata correlation
- Replay attacks
- Message tampering
- Identity key replacement
