# lockchain-core

Library of common types used in the `lockchain` crate ecosystem. It exposes traits and serde-compatible generic types that are meant to be embedded into various applications. Also provides stubbed functions with `unimplemented!()` macros to allow rapid prototyping or selective feature implementations.

The documentation isn't complete, if you have questions how to use it don't hesitate to open an issue. And docs PR's always welcome 💚.

## How to use

The core types provided by this crate are

- `Vault<T>`
- `Record<T>`
- `Body`

The last one is a trait that provides a few shared functions for backend implementations. Some might want to only keep encrypted data around while others could selectively query different fields. Some have more advanced versioning, some don't, etc. Additionally we have more types to round out a vault (`Payload`, `Header`, etc).

A simple implementation of a body type could look like this.

```rust
extern crate lockchain_core as lockchain;
use lockchain::traits::{Body, Encryption};

#[derive(Serialize, Deserialize)]
struct EncryptedBody {
    // ... your fields here ...
}

impl Body for EncryptedBody {

    // ... function implementations ...
}

impl Encryption for EncryptedBody {
    // ... implement the functions ...
}
```

## License

`lockchain` is free software: you can redistribute it and/or modify it under the terms of the MIT Public License.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the MIT Public License for more details.


## Conduct

In the interest of fostering an open and welcoming environment, the `lockchain` project pledges to making participation a harassment-free experience for everyone. See [Code of Conduct](../code_of_conduct.md) for details. In case of violations, e-mail [kookie@spacekookie.de](mailto:kookie@spacekookie.de).
