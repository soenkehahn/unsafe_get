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
