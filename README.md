# assert-size-derive

A Rust procedural macro for compile-time type size assertions.

## Overview

`assert-size-derive` provides a simple attribute macro that verifies types have the expected size in bytes at compile time. If the actual size doesn't match the expected size, compilation fails with a clear error message.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
assert-size-derive = "0.1.0"
```

## Usage

Apply the `#[assert_size(N)]` attribute to any type definition, where `N` is the expected size in bytes:

```rust
use assert_size_derive::assert_size;

#[assert_size(2)]
struct MyData {
    foo: u8,
    bar: u8,
}

#[assert_size(16)]
enum MyEnum {
    Variant1(u64),
    Variant2(u32),
}

#[assert_size(8)]
union MyUnion {
    unsigned: u64,
    signed: i64,
}
```

If the size doesn't match, you'll get a compile-time error:

```rust
// This will fail to compile!
#[assert_size(1)]
struct TooLarge {
    a: u8,
    b: u8,  // Actual size is 2 bytes
}
```

## Use Cases

- **Prevent regressions**: Catch unintended size changes from refactoring
- **FFI safety**: Ensure types meet specific memory layout requirements for C interop
- **Serialization**: Verify types have expected sizes for binary protocols
- **Performance**: Document and enforce size constraints for cache-friendly data structures
- **Cross-platform compatibility**: Detect platform-specific size variations

## Features

- ✅ Zero runtime overhead - all checks happen at compile time
- ✅ Works with structs, enums, and unions
- ✅ Supports all type attributes like `#[repr(C)]`, `#[repr(packed)]`, etc.
- ✅ Clear error messages on size mismatches
- ✅ Simple syntax - just one attribute with the expected size
- ✅ `no_std` compatible - works in embedded and bare-metal environments

## Examples

### With `repr` attributes

```rust
#[assert_size(15)]
#[repr(packed)]
struct PackedData {
    data: [u8; 11],
    ident: u32,
}

#[assert_size(1)]
#[repr(u8)]
enum SmallEnum {
    A,
    B,
    C,
}
```

### Zero-sized types

```rust
#[assert_size(0)]
struct Marker;

#[assert_size(0)]
struct EmptyStruct {}
```

### Tuple structs

```rust
#[assert_size(8)]
struct Wrapper(u64);
```

## How It Works

The macro generates a compile-time assertion using `core::mem::size_of` and const evaluation:

```rust
const _: () = assert!(EXPECTED_SIZE == ::core::mem::size_of::<YourType>());
```

The type definition itself is preserved unchanged, so there's no impact on the generated code. Using `core` ensures compatibility with both `std` and `no_std` environments.

## License

Licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
