## set_field: Rust derive macro for structs

Set fields on structs by string.

# SetField trait
```rust
pub trait SetField<T> {
    fn set_field(&mut self, field: &str, value: T) -> bool;
}
```

# Example

```rust
use set_field::SetField;

#[derive(SetField)]
struct Foo {
	a: i32,
	b: Option<bool>,
	c: i32,
}
fn test() {
	let mut t = Foo { a: 777, b: None, c: 0 };
	// return true on success:
	assert_eq!(t.set_field("a", 888), true);
	// return true on success:
	assert_eq!(t.set_field("b", Some(true)), true);
	assert_eq!(t.a, 888);
	assert_eq!(t.b, Some(true));
	// return false on nonexistent field:
	assert_eq!(t.set_field("d", 0), false);
	// return false on wrong type:
	assert_eq!(t.set_field("b", 0), false);
	// won't compile:
	// assert_eq!(t.set_field("a", 0.0), false);
}
```

* set_field returns true on success.
* set_field returns false if field doesn't exist.
* set_field returns false if you attempt to set a field to the wrong type.

# Expanded Foo struct from above

The SetField macro expands Foo into this: 

```rust
struct Foo {
	a: i32,
	b: Option<bool>,
	c: i32,
}
impl SetField<i32> for Foo {
    fn set_field(&mut self, field: &str, value: i32) -> bool {
        match field {
            "a" => {
                self.a = value;
                true
            }
            "c" => {
                self.c = value;
                true
            }
            _ => false,
        }
    }
}
impl SetField<Option<bool>> for Foo {
    fn set_field(&mut self, field: &str, value: Option<bool>) -> bool {
        match field {
            "b" => {
                self.b = value;
                true
            }
            _ => false,
        }
    }
}
```

# Dependencies

* [set_field_macro](https://crates.io/crates/set_field_macro)
* [syn](https://crates.io/crates/syn)
* [quote](https://crates.io/crates/quote)