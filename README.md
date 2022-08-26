# building
make sure you have a [rust toolchain](https://rustup.rs) installed

## [build](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
`cargo build --release --all-features`

## [docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
`cargo doc --all-features --no-deps`

if you want to open the docs after building

`cargo doc --all-features --no-deps --open`

# upgrading [`bb8-tiberius`] or [`tokio-util`]

Be careful when upgrading the [`bb8-tiberius`] or [`tokio-util`] crates (libraries) in [`Cargo.toml`].
Because there are trait implementations that works for both a [`tiberius::Client`] and a [`bb8::PooledConnection`],
the version of [`tokio-util`] in Cargo.toml must match the version of [`tokio-util`] that [`bb8-tiberius`] is using.

[`bb8::PooledConnection`]: (https://docs.rs/bb8/latest/bb8/struct.PooledConnection.html)
[`bb8-tiberius`]: (https://docs.rs/bb8-tiberius/latest/bb8_tiberius/)
[`tiberius::Client`]: (https://docs.rs/tiberius/latest/tiberius/struct.Client.html)
[`tokio-util`]: (https://docs.rs/tokio-util/latest/tokio_util/)
[`Cargo.toml`]: (Cargo.toml)
