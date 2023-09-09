//! Set struct fields by string
pub use set_field_macro::SetField;
pub trait SetField<T> {
    fn set_field(&mut self, field: &str, value: T) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::SetField;

    #[derive(SetField)]
    struct Foo {
        a: i32,
        b: Option<bool>,
        c: i32,
    }
    #[test]
    fn test() {
        let mut foo = Foo { a: 777, b: None, c: 0 };
        // return true on success:
        assert_eq!(foo.set_field("a", 888), true);
        // return true on success:
        assert_eq!(foo.set_field("b", Some(true)), true);
        assert_eq!(foo.a, 888);
        assert_eq!(foo.b, Some(true));
        // return false on nonexistent field:
        assert_eq!(foo.set_field("d", 0), false);
        // return false on wrong type:
        assert_eq!(foo.set_field("b", 0), false);
        // won't compile:
        // assert_eq!(t.set_field("a", 0.0), false);
    }
}