# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

HarProbe is a CLI tool that processes local HAR files. It has no network capabilities beyond reading files from disk.

If you discover a security vulnerability, please open an issue at https://github.com/EdgarOrtegaRamirez/harprobe/issues

## Security Features

- No external network requests
- No dependencies with native code
- File operations use safe Rust APIs
- Input validation on all CLI parameters
- No unsafe code blocks