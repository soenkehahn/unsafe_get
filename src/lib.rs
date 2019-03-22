#[macro_export]
macro_rules! get {
    ($value:expr, $constructor:path, $field:ident) => {{
        if let $constructor { $field, .. } = $value {
            $field
        } else {
            0
        }
    }};
}

#[cfg(test)]
mod get {
    #[test]
    fn returns_enum_fields() {
        enum Enum {
            Foo { foo: i32 },
            Bar,
        };
        assert_eq!(get!(Enum::Foo { foo: 42 }, Enum::Foo, foo), 42);
    }
}
