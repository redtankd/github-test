// Rust's deref coercions

#[cfg(test)]
mod test {

    struct Name {
        field: u32,
    }

    impl Name {
        fn f(&mut self) -> u32 {
            self.field += 1;
            self.field
        }

        fn ff(&self) -> u32 {
            self.field
        }
    }

    #[test]
    fn call_method() {
        let mut x = Name { field: 1 };

        assert_eq!((&mut x).f(), 2); // the regular calling
        assert_eq!(x.f(), 3); // deref coercions
        assert_eq!((&mut &mut x).f(), 4); // deref coercions

        assert_eq!((&x).ff(), 4); // the regular calling
        assert_eq!(x.ff(), 4); // deref coercions
        assert_eq!((&&x).ff(), 4); // deref coercions
        assert_eq!((&&&x).ff(), 4); // deref coercions
    }

    #[test]
    fn access_field() {
        let mut x = Name { field: 1 };

        assert_eq!(x.field, 1);
        assert_eq!((&x).field, 1); // deref coercions
        assert_eq!((&&x).field, 1); // deref coercions
        assert_eq!((&&&x).field, 1); // deref coercions

        x.field += 1;
        (&mut x).field += 1; // deref coercions
        (&mut &mut x).field += 1; // deref coercions
        (&mut &mut &mut x).field += 1; // deref coercions
        assert_eq!(x.field, 5);
    }
}
