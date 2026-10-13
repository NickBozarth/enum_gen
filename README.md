# EnumGen

A Rust procedural macro that automates discriminant generation for enum variants using a `const fn` generator.

## Features

* **Zero Runtime Overhead:** Computes values entirely at compile time using `const fn`.
* **Flexible Customization:** Configure your starting index, step size, and output representation type (`out_type`).
* **Clean Syntax:** Keeps your enums readable without manually typing repetitive discriminant expressions.

---

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
enum_gen = "0.1.0"

```

---

## Usage

Add the `#[enum_gen]` attribute macro to your enum, passing your generator function and any optional configuration arguments.

### Input Code

```rust
const fn generator(index: usize) -> usize {
    index * 100
}

#[enum_gen(generator, out_type = usize, start = 0usize, step = 1usize)]
enum MyEnum {
    V1,
    V2,
    V3,
}

```

### Expanded Output

The macro expands the enum at compile time into standard Rust with auto-incremented discriminant calls:

```rust
const fn generator(index: usize) -> usize {
    index * 100
}

#[repr(usize)]
enum MyEnum {
    V1 = generator(0usize),
    V2 = generator(1usize),
    V3 = generator(2usize),
}

```

---

## Configuration Arguments

| Argument | Default | Description |
| --- | --- | --- |
| `generator` *(required)* | — | The `const fn` used to calculate each discriminant value. |
| `out_type` | `usize` | The underlying representation type for the enum (automatically applies `#[repr(...)]`). |
| `start` | `0usize` | The initial index value passed to the first variant. |
| `step` | `1usize` | The increment step size applied between subsequent variants. |

---

## License

Licensed under either of the [Apache License, Version 2.0](https://www.google.com/search?q=LICENSE-APACHE) or [MIT License](https://www.google.com/search?q=LICENSE-MIT) at your option.
