# lockchain-core

If you're a Rust developer and interested in the `lockchain` crate, this README is for you. Lockchain is a document based, encrypted data vault. It provides you with an easy to use API to create, manage and update vaults and records. Build robust and user-friendly applications that deal with their data in a secure manner.

Most notibly, this crate is being used by the [lockchain]() password manager as well as the [poke]() linux ssh key manager.

## Example

Here is the most minimal example of how to use lockchain.

```Rust
use lockchain::record::Payload;
use lockchain::vault::Vault;

fn main() {
    let mut v = Vault::new(
        "user_secrets",
        "~/.local/my_app/",
        "user password here, it's hashed we promise",
    ).unwrap();

    /* Add a new record and put some data into it */
    v.add_record("User", "I say this is a category", None);
    v.add_data(
        "User",
        "data_key",
        Payload::Text(
            "This is some really important data that needs to be kept secure. Promise me!!!"
                .to_owned(),
        ),
    );

    /* All changes are kept in RAM until you sync */
    v.sync();
}
```

## Security notice

The cryptography in this crate has not undergone any formal review or verification. While stability and data integrity can be thoroughly tested, the security of this crate can not be guaranteed. **Use it at your own risk!**