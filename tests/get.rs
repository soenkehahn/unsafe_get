use unwrap_enum_field::{gimme, unwrap_enum_field};

#[derive(Debug)]
enum Enum {
    Foo { foo: i32 },
    Bar { bar: bool },
}

#[test]
fn returns_enum_fields() {
    assert_eq!(
        unwrap_enum_field!(Enum::Foo { foo: 42 }, Enum::Foo, foo),
        42
    );
}

#[test]
#[should_panic(
    expected = "unwrap_enum_field!: expected enum constructor: Enum::Foo, got Bar { bar: true }"
)]
fn panics_in_case_of_getting_passed_in_the_wrong_enum_constructor() {
    assert_eq!(
        unwrap_enum_field!(Enum::Bar { bar: true }, Enum::Foo, foo),
        42
    );
}

#[test]
fn works_for_different_types() {
    assert_eq!(
        unwrap_enum_field!(Enum::Bar { bar: true }, Enum::Bar, bar),
        true
    );
}

#[test]
fn gimme_works_like_unwrap_enum_field() {
    assert_eq!(gimme!(Enum::Foo { foo: 42 }, Enum::Foo, foo), 42);
}
