use unsafe_get::get;

#[derive(Debug)]
enum Enum {
    Foo { foo: i32 },
    Bar { bar: bool },
}

#[test]
fn returns_enum_fields() {
    assert_eq!(get!(Enum::Foo { foo: 42 }, Enum::Foo, foo), 42);
}

#[test]
#[should_panic(expected = "get!: expected enum constructor: Enum::Foo, got Bar { bar: true }")]
fn panics_in_case_of_getting_passed_in_the_wrong_enum_constructor() {
    assert_eq!(get!(Enum::Bar { bar: true }, Enum::Foo, foo), 42);
}

#[test]
fn works_for_different_types() {
    assert_eq!(get!(Enum::Bar { bar: true }, Enum::Bar, bar), true);
}
