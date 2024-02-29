This code is cross-compiled to a Raspberry Pi 4B device running Ubuntu. Install cargo-cross. Set the target as follows for both `random` and `blink`:

```bash
cross build --target aarch64-unknown-linux-gnu
```

On your Raspberry Pi 4B, create `pipe1` and `pipe1` in the same directory as the `random` and `blink` binaries. On your local device, `pipe1` and `pipe2` will be in the random-blink parent directory. You can generate these FIFO files using:

```bash
mkfifo pipe1
```

and

```bash
mkfifo pipe2
```
