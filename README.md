To see the issue:

```bash
cargo build
gcc test.c target/debug/liblinktest.dylib -o app && ./app
```

Change `crate_type = ["cdylib"]` to `crate_type = ["staticlib"]` in `Cargo.toml` and run:

```bash
cargo build
gcc test.c target/debug/liblinktest.a -o app && ./app
```
