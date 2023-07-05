# Jrest

[![Crates.io Package](https://img.shields.io/crates/v/jrest?style=for-the-badge)](https://crates.io/crates/jrest)
[![Docs.rs Documentation](https://img.shields.io/docsrs/jrest/latest?style=for-the-badge)](https://docs.rs/jrest/latest/jrest)
[![Test Workflow Status](https://img.shields.io/github/actions/workflow/status/ivangabriele/jrest/test.yml?label=CI&style=for-the-badge)](https://github.com/ivangabriele/jrest/actions?query=branch%3Amain+workflow%3ATest++)
[![Code Coverage](https://img.shields.io/codecov/c/github/ivangabriele/jrest/main?label=Cov&style=for-the-badge)](https://app.codecov.io/github/ivangabriele/jrest)

The equivalent of [Jest](https://jestjs.io) for Rust.

**Jrest** is a testing framework project for Rust, inspired by [Jest](https://jestjs.io),
easy to write and easy to read, with fancy diffs when tests fail.

> ⚠️ Be aware that this is a work in progress.
 
But it should get regular updates since I'm using it every week on my own open-source projects.

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
- [Thanks](#thanks)

---

## Installation

```sh
cargo add --dev jrest
```

## Usage

### `.to_be()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!("A &str").to_be("A &str");
      expect!("A String".to_string()).to_be("A String".to_string());
  }
}
```

### Primitives

#### `.to_be_greater_than()`

```rust
#[cfg(test)]
  mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!(3).to_be_greater_than(2);
  }
}
```

#### `.to_be_greater_than_or_equal()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!(3).to_be_greater_than_or_equal(2);
      expect!(3).to_be_greater_than_or_equal(3);
  }
}
```

#### `.to_be_less_than()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!(2).to_be_less_than(3);
  }
}
```

#### `.to_be_less_than_or_equal()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!(2).to_be_less_than_or_equal(3);
      expect!(2).to_be_less_than_or_equal(2);
  }
}
```

### Strings

#### `.to_start_with()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!("cargo").to_end_with("go");
  }
}
```

#### `.to_start_with()`

```rust
#[cfg(test)]
mod tests {
  use jrest::expect;

  #[test]
  fn test_something() {
      expect!("cargo").to_start_with("car");
  }
}
```

## Roadmap

- [ ] `.toContain()`
- [ ] `.toHaveLength()`
- [ ] `.toMatch()`
- [ ] `.toMatchObject()`
- [ ] `.toThrow()`

## Thanks

- [Tom Pridham](https://github.com/TomPridham)
  for [test-env-helpers](https://github.com/TomPridham/test-env-helpers) inspiration.
