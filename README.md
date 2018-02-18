# rust-enum-str-derive
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Build Status](https://travis-ci.org/victorporof/rust-enum-str-derive.svg?branch=master)](https://travis-ci.org/victorporof/rust-enum-str-derive)

A procedural macro derive allowing for automatically implementing `AsRef<str>` for enums.

## Purpose
Automate the tedious process of stringifying enum variants where needed.

## How to use
Procedural macros are not fully standardized as of September 2017, but sufficient features are available in the current rust nightly version (1.22). See the [RFC](https://github.com/rust-lang/rfcs/blob/master/text/1566-proc-macros.md) and the [tracking issue](https://github.com/rust-lang/rust/issues/38356) for more information.

Therefore, to create procedural macros and use this crate, you need Nightly:
```sh
rustup default nightly
```

Add this to your `Cargo.toml` file:

```toml
[dependencies]
enum-str-derive = { git = "https://github.com/victorporof/rust-enum-str-derive.git" }
```

Then, simply import the library into your code and derive the `EnumStr` trait on your data structures.

Available derives:
* `EnumStr`
* `EnumStrCamelCase`
* `EnumStrKebabCase`
* `EnumStrMixedCase`
* `EnumStrShoutySnakeCase`
* `EnumStrSnakeCase`
* `EnumStrTitleCase`

```rust
#![feature(proc_macro)]
extern crate enum_str_derive;

use enum_str_derive::{EnumStrSnakeCase};

#[derive(EnumStrCamelCase)]
enum Data {
    Foo,
    BarBaz
}

assert_eq!(Data::Foo.as_ref(), "foo");
assert_eq!(Data::BarBaz.as_ref(), "bar-baz");
```
