This code is cross-compiled to a Raspberry Pi 4B device running Ubuntu. Install cargo-cross. Set the target as follows for both `random` and `blink`:

```bash
cross build --target aarch64-unknown-linux-gnu
```

On your Raspberry Pi 4B, you can run the `blink` executable. This will call into `random` to generate a random number.

In order to modify the code on your local device, be sure to comment to remove the rppal dependency or the code will not compile.
