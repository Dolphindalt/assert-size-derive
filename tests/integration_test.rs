#![allow(unused)]
use assert_size_derive::assert_size;

// Basic struct tests
#[assert_size(2)]
struct MyData {
    foo: u8,
    bar: u8,
}

#[assert_size(15)]
#[repr(packed)]
struct MoreComplexData {
    data: [u8; 11],
    ident: u32,
}

// Enum tests
#[assert_size(1)]
#[repr(u8)]
enum Datum {
    Test,
    Awesome,
}

#[assert_size(16)]
enum LargeEnum {
    Variant1(u64),
    Variant2(u32),
}

// Union tests
#[assert_size(8)]
union MyUnion {
    unsigned: u64,
    signed: i64,
}

#[assert_size(4)]
union SmallUnion {
    byte: u8,
    word: u32,
}

// Zero-sized types
#[assert_size(0)]
struct ZeroSized;

#[assert_size(0)]
struct EmptyStruct {}

#[assert_size(0)]
enum EmptyEnum {}

// Generic types - test with concrete instantiations
struct GenericStruct<T> {
    value: T,
}

#[assert_size(8)]
struct ConcreteGenericU64 {
    inner: GenericStruct<u64>,
}

#[assert_size(1)]
struct ConcreteGenericU8 {
    inner: GenericStruct<u8>,
}

struct GenericPair<T, U> {
    first: T,
    second: U,
}

#[assert_size(16)]
struct ConcretePairU64U64 {
    inner: GenericPair<u64, u64>,
}

#[assert_size(2)]
struct ConcretePairU8U8 {
    inner: GenericPair<u8, u8>,
}

// Edge cases
#[assert_size(1)]
struct SingleByte(u8);

#[assert_size(0)]
struct TupleZeroSized();

#[assert_size(24)]
struct NestedStruct {
    inner: MyData,
    extra: [u64; 2],
    byte: u8,
    // 2 bytes from MyData + 5 bytes padding + 16 bytes for [u64; 2] + 1 byte = 24
}
