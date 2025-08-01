# irmin-rs

<a href="https://crates.io/crates/irmin">
    <img src="https://img.shields.io/crates/v/irmin.svg">
</a>

[irmin](https://irmin.org) bindings for Rust

[Documentation](https://docs.rs/irmin)

This crate enables you to call directly into irmin from your Rust application and
can be used to open an existing irmin store from Rust that may have been created
by an application written in OCaml.

## Building

After installing [libirmin](https://github.com/mirage/irmin) using opam, you can run:

```
$ cargo build
```

And the build script should be able to find the location of the `libirmin` library and header files.

If `libirm.so` and `irmin.h` were not installed using opam and they're not in `~/.local` or
`/usr/local`, then you can specify where to look for them using the `LIBIRMIN_PREFIX` env
variable.

### Nix Users

Make sure you have [flake features](https://nixos.wiki/wiki/Flakes) enabled, then after installing [libirmin](https://github.com/mirage/irmin) using opam, you can then run a shell with [direnv](https://direnv.net/).
```
$ direnv allow
```

## Testing

> [!WARNING]
> The automated tests in this crate (`cargo test`) currently fail due to incompatibilities between Rust's 
> test harness and `OCaml 5.x` domain system. The OCaml runtime produces "no domain lock held" errors when 
> initialized within Rust's test execution context. This does not affect normal usage - the library works 
> correctly when used in regular Rust applications with a standard `main()`s function.

Tests must be executed using a single thread:

```
$ cargo test -- --test-threads=1
```

or

```
$ make test
```
