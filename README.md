# derive_newtype

## About

This derive macro provides `Deref`, `DerefMut`, and `From` for the newtype pattern.

A similar crate exists but has not been updated.

## Notice

This library has no plans to add further functionality at this time. Only version updates of dependent libraries will be followed.

## How to use

```rust
use derive_newtype::Newtype;

#[derive(Newtype)]
struct Foo(u8);
```

This macro will generate the following code:

```rust
impl core::ops::Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<u8> for Foo {        
    fn from(inner: u8) -> Foo {
        Foo(inner)
    }
}
```

## License

derive_newtype is released under the MIT License
