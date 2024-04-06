# ohook.rs

A rewrite of [ohook](https://github.com/asdcorp/ohook) in Rust for decreased size, easier building, and better reproducibility. For more information on ohook check the original repo.

## Building
Rust 1.77.1 and [Visual Studio Build Tools 2022 LTSC 17.6](https://download.visualstudio.microsoft.com/download/pr/a851fc84-7739-4b67-a7da-2c8564e30b38/b4133f16d790c3ee7325fff80c47094d94dff44b426b86db9013b200bb669ce2/vs_BuildTools.exe) running on Windows is required if you want to build binaries bit-for-bit identical to the release. To build x64, run `cargo build --release`, and for x86 run `cargo build --release --target i686-pc-windows-msvc`.

## License

The project is licensed under the terms of the MIT License.
