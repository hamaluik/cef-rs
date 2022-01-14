#[cfg(test)]
mod tests {
    use cef_ref_counting::{ref_count, RefCount};

    #[test]
    fn adds_ref_count_field() {
        #[ref_count]
        #[derive(Debug)]
        struct Foo {}

        let bar = Foo {
            ref_count: std::sync::atomic::AtomicIsize::new(0),
        };

        assert_ne!(std::mem::size_of_val(&bar), 0);
    }

    #[test]
    fn keeps_original_field() {
        #[ref_count]
        #[derive(Debug)]
        struct Foo {
            a: String,
        }

        let bar = Foo {
            a: "lorem ipsum".to_string(),
            ref_count: std::sync::atomic::AtomicIsize::new(0),
        };

        assert_eq!(bar.a, "lorem ipsum");
    }

    #[test]
    fn adds_field_with_derive() {
        #[ref_count]
        #[derive(Debug, RefCount)]
        struct Foo {
            a: String,
        }

        let bar = Foo {
            a: "lorem ipsum".to_string(),
            ref_count: std::sync::atomic::AtomicIsize::new(0),
        };

        assert_eq!(bar.a, "lorem ipsum");
    }
}
