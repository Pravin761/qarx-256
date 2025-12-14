# Security Notice

## ⚠️ EXPERIMENTAL - NOT FOR PRODUCTION USE

QARX-256 is an **experimental cryptographic cipher** designed for research, learning, and lab purposes only.

### DO NOT USE for:
- Real data encryption
- Passwords
- Financial data
- Production systems
- Any security-critical applications

### SAFE TO USE for:
- Cryptographic research
- Educational purposes
- CTF (Capture The Flag) challenges
- Malware/ransomware simulation (in controlled, sandboxed environments)
- Red team exercises (within authorized scope)
- Academic papers and cryptanalysis studies

## Why It's Not Production-Ready

1. **No Cryptographic Audit** - This cipher has NOT been reviewed by independent cryptographers
2. **No Public Cryptanalysis** - Unknown resistance to modern cryptographic attacks
3. **Experimental Design** - ARX construction is novel and unproven
4. **Side-Channel Vulnerabilities** - Not hardened against timing or power analysis attacks
5. **No Standardization** - Not endorsed by NIST, IETF, or any standards body

## Vulnerability Disclosure

If you find a vulnerability or weakness in this cipher:

1. **DO NOT** open a public GitHub issue disclosing the vulnerability
2. **DO** use the GitHub Security Advisories feature for responsible disclosure
3. Please provide:
   - Description of the vulnerability
   - Impact assessment
   - Proof of concept (if applicable)
   - Recommended fix

This is a research project, so vulnerabilities are expected and welcomed as learning opportunities.

## Comparison with Production Ciphers

For production use, consider:
- **AES-256-GCM** (NIST standardized, battle-tested)
- **ChaCha20-Poly1305** (RFC 8439, widely used)
- **Kyber** (Post-quantum candidate)

## Current Known Issues

- No constant-time guarantees
- No protection against timing attacks
- Timing-variable entropy threshold in modes
- No side-channel resistance

## Legal Disclaimer

THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

The authors assume no liability for damages caused by use of this experimental software.

## Questions?

For research discussions, open an issue. For security concerns, use GitHub Security Advisories.
