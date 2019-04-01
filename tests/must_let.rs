#![allow(clippy::blacklisted_name)]

use unsafe_get::must_let;

#[derive(Debug)]
enum Enum {
    Foo { foo: i32 },
    Bar { bar: bool },
    Baz { a: i32, b: i32 },
}

#[test]
fn returns_enum_fields() {
    let value = Enum::Foo { foo: 42 };
    must_let!(Enum::Foo { foo } = value);
    assert_eq!(foo, 42);
}

#[test]
#[should_panic(expected = "must_let!: expected enum constructor: Enum::Foo, got Bar { bar: true }")]
fn panics_in_case_of_getting_passed_in_the_wrong_enum_constructor() {
    let value = Enum::Bar { bar: true };
    must_let!(Enum::Foo { foo } = value);
    assert_ne!(foo, 1); // use `foo` to suppress unused variable warning
}

#[test]
fn works_for_different_types() {
    let value = Enum::Bar { bar: true };
    must_let!(Enum::Bar { bar } = value);
    assert_eq!(bar, true);
}

#[test]
fn binds_multiple_variables() {
    let value = Enum::Baz { a: 3, b: 4 };
    must_let!(Enum::Baz { a, b } = value);
    assert_eq!((a, b), (3, 4));
}

#[test]
fn accepts_pattern_with_spread() {
    let value = Enum::Baz { a: 3, b: 4 };
    must_let!(Enum::Baz { a, .. } = value);
    assert_eq!(a, 3);
}

#[test]
fn matches_tuple_constructor() {
    let value = Some(42);
    must_let!(Some(x) = value);
    assert_eq!(x, 42);
}

#[test]
fn matches_tuple_constructor_with_path() {
    let value = Some(42);
    must_let!(std::option::Option::Some(x) = value);
    assert_eq!(x, 42);
}

#[test]
fn binds_field_value_to_arbitrary_variable_name() {
    let value = Enum::Foo { foo: 42 };
    must_let!(Enum::Foo { foo: x } = value);
    assert_eq!(x, 42);
}

#[test]
fn binds_multiple_field_values_to_custom_variables() {
    let value = Enum::Baz { a: 3, b: 4 };
    must_let!(
        Enum::Baz {
            a: value_a,
            b: value_b
        } = value
    );
    assert_eq!((value_a, value_b), (3, 4));
}
