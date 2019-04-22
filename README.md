# lockchain

A modern, cross-platform and extendable secrets manager toolchain.

Lockchain is a series of interconnected libraries that provide
utilities and abstraction layers to build distributed secret
managers. It's primary application is the [`pwchain`] password manager.

| Component Crate    | Description                               |
|--------------------|-------------------------------------------|
| [lockchain-core]   | Core ecosystem interface and utilities    |
| [lockchain-crypto] | Crypto engine handler                     |
| [lockchain-files]  | File-based storage backend                |
| [lockchain-http]   | Slim http-layer API on top of Vault API   |

[lockchain-core]: lockchain-core/
[lockchain-crypto]: lockchain-crypto/
[lockchain-files]: lockchain-files/
[lockchain-http]: lockchain-http/
[`pwchain`]: https://github.com/spacekookie/pwchain

## Security notice

The cryptography in this crate has not undergone any formal review or verification. While stability and data integrity can be thoroughly tested, the security of this crate can not be guaranteed. **Use it at your own risk!**
