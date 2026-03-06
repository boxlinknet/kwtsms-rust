# Security Policy

## Reporting a Vulnerability

Do **not** open a public issue for security vulnerabilities.

Instead, use one of these methods:

1. **GitHub Security Advisory** (preferred): [Create a private advisory](https://github.com/boxlinknet/kwtsms-rust/security/advisories/new)
2. **Direct contact**: Reach out via [kwtSMS support](https://www.kwtsms.com/support.html)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

We will acknowledge receipt within 48 hours and provide a timeline for a fix.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Security Best Practices

When using this library:

- **Never hardcode** API credentials in source code
- **Use environment variables** or a secrets manager for credentials
- **Enable CAPTCHA** on all forms that trigger SMS sends
- **Rate limit** per phone number, per IP, and per user
- **Monitor** for abuse patterns and rapid balance depletion
- See the [Security Checklist](README.md#security-checklist) in the README
