use num::ToPrimitive;
use std::{fmt::Debug, panic::Location};

// macro_rules! implement_expectation_shared_methods {
//     ($trait_fragment:tt$(<$type_fragment:tt>)?) => {
//         impl<T: $trait_fragment$(<$type_fragment>)? + Debug> Expectation<T> {
//             fn panic_with_assertion(&self, right_value: T, comparator: &str) {
//                 panic!(
//                     "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`",
//                     comparator, self.left_value, right_value,
//                 );
//             }
//         }
//     };
// }

// pub fn expect<T: Debug>(left_value: T) -> Expectation<T> {
//     Expectation::new(left_value)
// }

/// Wrap the result to be tested, giving you access to [Expectation] matchers (methods).
///
/// # Examples
///
/// ```
/// use jrest::expect;
///
/// fn sum(a: i32, b: i32) -> i32 {
///    return a + b;
/// }
///
/// expect!(sum(1, 2)).to_be(3);
/// ```
#[macro_export]
macro_rules! expect {
    ($left_value:expr) => {{
        jrest::Expectation::new($left_value)
    }};
}

pub struct Expectation<L: Debug> {
    at_path: &'static str,
    at_line: u32,
    left_value: L,
}
impl<L: Debug> Expectation<L> {
    #[doc(hidden)]
    #[track_caller]
    pub fn new(left_value: L) -> Self {
        let at_path = Location::caller().file();
        let at_line = Location::caller().line();

        Self {
            at_line,
            at_path,
            left_value,
        }
    }
}
impl<L: Debug> Expectation<L> {
    fn panic_with_assertion<R: Debug>(&self, right_value: R, comparator: &str) {
        panic!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`\nat {}:{}\n\n",
            comparator, self.left_value, right_value, self.at_path, self.at_line
        );
    }
}

impl<L: Debug + Eq + PartialEq> Expectation<L> {
    /// Expect _result_ to exactly equal _expectation_.
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!(2).to_be(2);
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!(2).to_be(3);
    /// ```
    pub fn to_be(&self, right_value: L) {
        if !(self.left_value == right_value) {
            self.panic_with_assertion(right_value, "===");
        }
    }

    // pub fn to_equal<R: 'static + Debug + Eq + Into<L> + PartialEq>(&self, right_value: &R)
    // where
    //     &'static R: Into<&'static L>,
    // {
    //     if !(&self.left_value == right_value.into()) {
    //         self.panic_with_assertion(right_value, "==");
    //     }
    // }
}

// implement_expectation_shared_methods! { ToPrimitive }
impl<T: ToPrimitive + Debug> Expectation<T> {
    /// Expect _result_ to be greater than _expectation_.
    ///
    /// This works with any type implementing [`num::ToPrimitive`]
    /// (likely any kind of number: `f8`, `i16`, `u32`, `usize`, etc).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_greater_than(1);
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_greater_than(2);
    /// expect!(2).to_be_greater_than(3);
    /// ```
    pub fn to_be_greater_than(&self, right_value: T) {
        if !(self.left_value.to_f32() > right_value.to_f32()) {
            self.panic_with_assertion(right_value, ">");
        }
    }

    /// Expect _result_ to be greater than or equal to _expectation_.
    ///
    /// This works with any type implementing [`num::ToPrimitive`]
    /// (likely any kind of number: `f8`, `i16`, `u32`, `usize`, etc).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_greater_than_or_equal(1);
    /// expect!(2).to_be_greater_than_or_equal(2);
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_greater_than_or_equal(3);
    /// ```
    pub fn to_be_greater_than_or_equal(&self, right_value: T) {
        if !(self.left_value.to_f32() >= right_value.to_f32()) {
            self.panic_with_assertion(right_value, ">=");
        }
    }

    /// Expect _result_ to be less than _expectation_.
    ///
    /// This works with any type implementing [`num::ToPrimitive`]
    /// (likely any kind of number: `f8`, `i16`, `u32`, `usize`, etc).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_less_than(3);
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_less_than(2);
    /// expect!(2).to_be_less_than(1);
    /// ```
    pub fn to_be_less_than(&self, right_value: T) {
        if !(self.left_value.to_f32() < right_value.to_f32()) {
            self.panic_with_assertion(right_value, "<");
        }
    }

    /// Expect _result_ to be less than or equal to _expectation_.
    ///
    /// This works with any type implementing [`num::ToPrimitive`]
    /// (likely any kind of number: `f8`, `i16`, `u32`, `usize`, etc).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_less_than_or_equal(3);
    /// expect!(2).to_be_less_than_or_equal(2);
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!(2).to_be_less_than_or_equal(1);
    /// ```
    pub fn to_be_less_than_or_equal(&self, right_value: T) {
        if !(self.left_value.to_f32() <= right_value.to_f32()) {
            self.panic_with_assertion(right_value, "<=");
        }
    }
}

// implement_expectation_shared_methods! { AsRef<str> }
impl<T: AsRef<str> + Debug> Expectation<T> {
    /// Expect _result_ to end with _expectation_.
    ///
    /// This works with any type implementing `AsRef<str>`
    /// (likely [`String`] & [`str`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!("abc").to_end_with("bc");
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!("abc").to_end_with("ab");
    /// ```
    pub fn to_end_with<R: AsRef<str> + Debug>(&self, right_value: R) {
        if !self.left_value.as_ref().ends_with(right_value.as_ref()) {
            self.panic_with_assertion(right_value, "ends with");
        }
    }

    /// Expect _result_ to start with _expectation_.
    ///
    /// This works with any type implementing `AsRef<str>`
    /// (likely [`String`] & [`str`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use jrest::expect;
    ///
    /// expect!("abc").to_start_with("ab");
    /// ```
    ///
    /// ```should_panic
    /// use jrest::expect;
    ///
    /// expect!("abc").to_start_with("bc");
    /// ```
    pub fn to_start_with<R: AsRef<str> + Debug>(&self, right_value: R) {
        if !self.left_value.as_ref().starts_with(right_value.as_ref()) {
            self.panic_with_assertion(right_value, "starts with");
        }
    }
}
