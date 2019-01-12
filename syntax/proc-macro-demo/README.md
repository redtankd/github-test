# Procedural Macros

From Rust's Book

> There are three kinds of procedural macros, but they all work in a similar fashion. First, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future.

```toml
[lib]
proc-macro = true
```

> Second, defining the macro with a function, which has an attribute like `some_attribute`. This attribute says which kind of procedural macro we’re creating.

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

## Three kinds of procedural macros

* Derive macros: only works for structs and enums

* Attribute-like macros

* Function-like macros

## Helper crate

`syn`, `quote`, and `proc-macro2` are your go-to libraries for writing procedural macros. They make it easy to define custom parsers, parse existing syntax, create new syntax, work with older versions of Rust, and much more!