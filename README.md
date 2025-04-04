Meson
![license](https://img.shields.io/github/license/hqnna/zed-meson)
![activity](https://img.shields.io/github/last-commit/hqnna/zed-meson)
================================================================================

> [!IMPORTANT]
> This extension is in **maintenance** mode, it will only be updated to fix breakages.

An extension for the [zed](https://zed.dev) code editor, adding support for the
[meson](https://mesonbuild.com) build system.

## Developing the extension

When testing the extension you will want to install the *directory* as a dev
extension in your editor.

## Building from source

To build this extension from source you'll need the latest stable version of the
[Rust](https://www.rust-lang.org/) toolchain.

```console
$ cargo build
  Downloaded zed_extension_api v0.3.0
  Downloaded 1 crate (18.3 KB) in 0.14s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.40s
```

This will produce a shared library located in `target/debug/libzed_meson.so`.
