//! This crate provides a non-total way to access enum fields.
//!
//! # Motivation
//!
//! In most cases you should use pattern matching to deconstruct enums.
//! Pattern matching forces us to handle every enum case, and through that
//! eliminates a whole category of runtime bugs. This crate provides an escape
//! hatch to that, and shouldn't be used in normal circumstances. There's one
//! situation where `unwrap_enum_field!` might be useful though: tests.
//!
//! Here's an example of an enum and a function that returns a value of that enum:
//! ```
//! #[derive(Debug, PartialEq)]
//! enum ExampleEnum {
//!   Foo { a: i32, b: i32 },
//!   Bar,
//! }
//!
//! fn computes_something() -> ExampleEnum {
//!   ExampleEnum::Foo { a: 1, b: 2 }
//! }
//! ```
//! Imagine you want to write a test for the correctness of the field `b` in the
//! return value of `computes_something`. You could do it like this:
//! ```
//! # #[derive(Debug, PartialEq)]
//! # enum ExampleEnum {
//! #   Foo { a: i32, b: i32 },
//! #   Bar,
//! # }
//! #
//! # fn computes_something() -> ExampleEnum {
//! #   ExampleEnum::Foo { a: 1, b: 2 }
//! # }
//! #
//! fn computes_something_returns_correct_b() {
//!   assert_eq!(
//!     computes_something(),
//!     ExampleEnum::Foo { a: 1, b: 2 }
//!   )
//! }
//! ```
//! But that is actually also comparing the value of field `a` and therefore
//! is testing more than you want to test. And that's not ideal. Instead,
//! you could do this:
//! ```
//! # #[derive(Debug, PartialEq)]
//! # enum ExampleEnum {
//! #   Foo { a: i32, b: i32 },
//! #   Bar,
//! # }
//! #
//! # fn computes_something() -> ExampleEnum {
//! #   ExampleEnum::Foo { a: 1, b: 2 }
//! # }
//! #
//! fn computes_something_returns_correct_b() {
//!   match computes_something() {
//!     ExampleEnum::Foo { b, .. } =>
//!       assert_eq!(b, 2),
//!     ExampleEnum::Bar =>
//!       panic!("shouldn't be Bar"),
//!   }
//! }
//! ```
//! That only tests what you want to test, but is also a bit ugly. With `unwrap_enum_field!`
//! you can write:
//! ```
//! # #[derive(Debug, PartialEq)]
//! # enum ExampleEnum {
//! #   Foo { a: i32, b: i32 },
//! #   Bar,
//! # }
//! #
//! # fn computes_something() -> ExampleEnum {
//! #   ExampleEnum::Foo { a: 1, b: 2 }
//! # }
//! #
//! use unwrap_enum_field::unwrap_enum_field;
//!
//! fn computes_something_returns_correct_b() {
//!   assert_eq!(unwrap_enum_field!(computes_something(), ExampleEnum::Foo, b), 2);
//! }
//! ```

/// The `unwrap_enum_field!` macro provides a non-total way to access enum fields:
///
/// ```
/// use unwrap_enum_field::unwrap_enum_field;
///
/// #[derive(Debug)]
/// enum ExampleEnum {
///   Foo { field: i32 },
///   Bar { other_field: String },
/// }
///
/// let value = ExampleEnum::Foo { field: 42 };
/// assert_eq!(unwrap_enum_field!(value, ExampleEnum::Foo, field), 42);
/// ```
///
/// If the first argument to `unwrap_enum_field!` is constructed with a different
/// constructor than the one passed in as the second argument, `unwrap_enum_field!`
/// will panic.
///
/// ```should_panic
/// use unwrap_enum_field::unwrap_enum_field;
///
/// #[derive(Debug)]
/// enum ExampleEnum {
///   Foo { field: i32 },
///   Bar { other_field: String },
/// }
///
/// let value = ExampleEnum::Foo { field: 42 };
/// let other_field = unwrap_enum_field!(value, ExampleEnum::Bar, other_field); // panics
/// ```
#[macro_export]
macro_rules! unwrap_enum_field {
    ($value:expr, $constructor:path, $field:ident) => {{
        if let $constructor { $field, .. } = $value {
            $field
        } else {
            panic!(
                "unwrap_enum_field!: expected enum constructor: {}, got {:?}",
                stringify!($constructor),
                $value
            )
        }
    }};
}
