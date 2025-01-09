# Building Locally

To build the project locally, you will need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [llvm](https://releases.llvm.org/download.html)
- [lld](https://lld.llvm.org/)

Once you have Rust + LLD + LLVM installed, you can build the project by running:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/cli`.

> [!NOTE]
> For actual use, you'll want to alias the binary to `cloudstate` for ease of use.

## Using as a library

The single binary is **not meant to be used as a library**. If you want to use the cloudstate library in your project, you should use either use the `server` or `runtime` crate. The server crate exposes an http server that can be embedded in your project, while the runtime crate exposes the core functionality of cloudstate.

> [!NOTE]
> Cloudstate uses the tokio `tracing` crate for logging. If you want to get these logs, you'll need to set up your own `tracing-subscriber` in your project.
