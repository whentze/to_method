# to_method

A utility micro-crate for using [`Into`](::core::convert::Into) more ergonomically.

It exposes a [`To`](crate::To) extension trait with a [`.to()`](To::to) method
which you can use to invoke [`Into::into`](::core::convert::Into::into)
while specifying the target type and without having to abandon method-call syntax.

Being a micro-crate, it tries to be as nice of a dependency as possible and has:

- No dependencies of its own
- No feature flags
- No `build.rs`
- `#![no_std]`
- `#![forbid(unsafe_code)]`

## Regular `Into` usage

```rust
let x : u8 = 5;

// The type parameter is on `Into`, not on `Into::into`,
// so we need to do it like this:
let y = Into::<u16>::into(x);

// Depending on context, inference can make this work though:
let z : u32 = y.into();
```

## With `To`

```rust
use to_method::To as _;

let x : u8 = 5;

// The type parameter is on the `to` method, so this works:
let y = x.to::<u16>();

// And you can still rely on inference as well:
let z : u32 = y.to();
```
