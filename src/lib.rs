/// The `get!` macro provides a non-total way to access enum fields:
///
/// ```
/// use unsafe_get::get;
///
/// #[derive(Debug)]
/// enum ExampleEnum {
///   Foo { field: i32 },
///   Bar { other_field: String },
/// }
///
/// let value = ExampleEnum::Foo { field: 42 };
/// assert_eq!(get!(value, ExampleEnum::Foo, field), 42);
/// ```
///
/// If the first argument to `get!` is constructed with a different
/// constructor than the one passed in as the second argument, `get!`
/// will panic.
///
/// ```should_panic
/// use unsafe_get::get;
///
/// #[derive(Debug)]
/// enum ExampleEnum {
///   Foo { field: i32 },
///   Bar { other_field: String },
/// }
///
/// let value = ExampleEnum::Foo { field: 42 };
/// let other_field = get!(value, ExampleEnum::Bar, other_field); // panics
/// ```
#[macro_export]
macro_rules! get {
    ($value:expr, $constructor:path, $field:ident) => {{
        if let $constructor { $field, .. } = $value {
            $field
        } else {
            panic!(
                "get!: expected enum constructor: {}, got {:?}",
                stringify!($constructor),
                $value
            )
        }
    }};
}
