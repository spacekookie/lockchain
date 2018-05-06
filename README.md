# lockchain

A modern, cross-platform and extendable secrets manager.

## To own the libs

This library ecosystem is made of three main parts.

 - `lockchain-core` Common types shared between both `secret` and `default` operation modes.
 - `lockchain-files` load vaults, decode files and work with encrypted streams.
 - `lockchain-crypto` attach crypto handlers to vaults as a middleware to decrypt data.

This enables a few different configurations.

- `lockchain-server` uses `lockchain-core` and `lockchain-files` to serve encrypted files to various front-ends (maybe even on the same machine)
- `lockchain-client` uses `lockchain-core` and `lockchain-crypto` to decrypt received data from a server and use it in a front-end application.

Additionally there is `lockchain-http` which provides an easy to use RESTful API to use for browser extentions or client-side logic which can't rely on local cryptography. 


## Security notice

The cryptography in this crate has not undergone any formal review or verification. While stability and data integrity can be thoroughly tested, the security of this crate can not be guaranteed. **Use it at your own risk!**