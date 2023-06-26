# Rest

[![Test Workflow Status](https://img.shields.io/github/actions/workflow/status/ivangabriele/rest/test.yml?label=Tests&style=for-the-badge)](https://github.com/ivangabriele/rest/actions?query=branch%3Amain+workflow%3ATest++)
[![Code Coverage](https://img.shields.io/codecov/c/github/ivangabriele/rest/main?style=for-the-badge)](https://app.codecov.io/github/ivangabriele/rest)

The equivalent of [Jest](https://jestjs.io) for Rust.

Rest is a testing framework for Rust, inspired by [Jest](https://jestjs.io),
easy to write and easy to read, with diffs when tests fail.

> ⚠️ This is a work in progress.

---

- [Installation](#installation)
- [Usage](#usage)
  - [`.to_be()`](#to_be)
  - [Primitives](#primitives)
    - [`.to_be_greater_than()`](#to_be_greater_than)
    - [`.to_be_greater_than_or_equal()`](#to_be_greater_than_or_equal)
    - [`.to_be_less_than()`](#to_be_less_than)
    - [`.to_be_less_than_or_equal()`](#to_be_less_than_or_equal)
  - [Strings](#strings)
    - [`.to_start_with()`](#to_start_with)
    - [`.to_start_with()`](#to_start_with-1)
- [Roadmap](#roadmap)

---

## Installation

```sh
cargo add --dev --git https://github.com/ivangabriele/rest
```

> **Note**<br>
> [https://crates.io/crates/rest](https://crates.io/crates/rest) is unfortunately squatted by a ghost.<br>
> That's why you need to add this GitHub repository instead.

## Usage

### `.to_be()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!("A &str").to_be("A &str");
    expect!("A String".to_string()).to_be("A String".to_string());
}
```

### Primitives

#### `.to_be_greater_than()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!(3).to_be_greater_than(2);
}
```

#### `.to_be_greater_than_or_equal()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!(3).to_be_greater_than_or_equal(2);
    expect!(3).to_be_greater_than_or_equal(3);
}
```

#### `.to_be_less_than()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!(2).to_be_less_than(3);
}
```

#### `.to_be_less_than_or_equal()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!(2).to_be_less_than_or_equal(3);
    expect!(2).to_be_less_than_or_equal(2);
}
```

### Strings

#### `.to_start_with()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!("cargo").to_end_with("go");
}
```

#### `.to_start_with()`

```rust
use rest::expect;

#[test]
fn test_something() {
    expect!("cargo").to_start_with("car");
}
```

## Roadmap

- [ ] `.toContain()`
- [ ] `.toHaveLength()`
- [ ] `.toMatch()`
- [ ] `.toMatchObject()`
- [ ] `.toThrow()`
