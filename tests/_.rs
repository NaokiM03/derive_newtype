#[cfg(test)]
mod tests {
    use derive_newtype::Newtype;

    #[derive(Debug, Newtype, PartialEq)]
    struct Simple(u8);

    #[test]
    fn simple() {
        // Deref
        fn deref_f(_: &u8) {}
        let i = u8::default();
        let simple = Simple(i);
        deref_f(&simple);

        // DerefMut
        let i = u8::default();
        let mut simple = Simple(i);
        *simple += 1;
        assert_eq!(simple, Simple(1));

        // From
        let i = u8::default();
        let _: Simple = i.into();
    }

    #[derive(Debug, Newtype, PartialEq)]
    struct WithGenerics<T>(T);

    #[test]
    fn with_generics() {
        let i = u8::default();
        let _ = WithGenerics(i);

        let i = usize::default();
        let _ = WithGenerics(i);

        let s = String::default();
        let _ = WithGenerics(s);

        let v = vec!["".to_owned()];
        let _ = WithGenerics(v);

        // Deref
        fn deref_f(_: &i32) {}
        let i = i32::default();
        let simple = WithGenerics(i);
        deref_f(&simple);

        // DerefMut
        let i = i32::default();
        let mut simple = WithGenerics(i);
        *simple += 1;
        assert_eq!(simple, WithGenerics(1));

        // From
        let i = i32::default();
        let _: WithGenerics<i32> = i.into();
    }
}
