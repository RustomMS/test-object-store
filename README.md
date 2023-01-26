To repros a issue with Google Cloud Storage and `ObjectStore::rename_if_not_exists` resulting in 411 because the header doesn't contain the content length

Repro can be run with
```
cargo test
```

Fails with the following conditions as set by you in the Cargo.toml
```
# Fail defult-tls and native-tls both fail the test
reqwest = { version = "0.11.14", default-features = false, features = ["default-tls"] }
reqwest = { version = "0.11.14", default-features = false, features = ["native-tls"] }
```

Passes with the following conditions
With `default-tls` and the patched `object_store`
```
reqwest = { version = "0.11.14", default-features = false, features = ["default-tls"] }

[patch.crates-io]
#object_store = { git = "https://github.com/glaredb/arrow-rs.git", branch = "content-length" }
```

With `native-tls` and the patched `object_store`
```
reqwest = { version = "0.11.14", default-features = false, features = ["native-tls"] }

[patch.crates-io]
#object_store = { git = "https://github.com/glaredb/arrow-rs.git", branch = "content-length" }
```
And with `rustls-tls` (this is what is used in the `object_store` crate
```
reqwest = { version = "0.11.14", default-features = false, features = ["rustls-tls"] }
```

