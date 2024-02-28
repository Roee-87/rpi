This code is cross-compiled to a Raspberry Pi 4 B device running Ubuntu.
Install cargo-cross.
Set the target as follows:

```bash
cross build --target aarch64-unknown-linux-gnu
```
The binary is can be found in `target/aarch64-unknown-linux-gnu/debug/blink-led`

