# rust-raytracing

[_Ray Tracing in One Weekend - The Book Series_](https://raytracing.github.io/) - implemented in Rust.

## Run

- Windows Powershell

```powershell
git clone git@github.com:MizukiCry/rust-raytracing.git
cd rust-raytracing
cargo run --release | out-file output.ppm -encoding ascii
```

- Linux

```bash
git clone git@github.com:MizukiCry/rust-raytracing.git
cd rust-raytracing
cargo run --release > output.ppm
```
