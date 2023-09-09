# Derive macro for set_field
see [set_field](https://crates.io/crates/set_field)

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