#[cfg(test)]
mod tests {
    use cef97_sys::cef_base_ref_counted_t;
    use cef_ref_counting::{ref_count, RefCount};

    #[test]
    fn impls_ref_counting_methods() {
        let _ = env_logger::builder().is_test(true).try_init();

        #[ref_count]
        #[derive(Debug, RefCount)]
        #[allow(dead_code)]
        struct Foo {
            base: cef_base_ref_counted_t,
        }

        let foo = Foo {
            base: cef_base_ref_counted_t {
                size: std::mem::size_of::<Foo>() as u64,
                add_ref: Some(Foo::add_ref),
                release: Some(Foo::release),
                has_one_ref: Some(Foo::has_one_ref),
                has_at_least_one_ref: Some(Foo::has_at_least_one_ref),
            },
            ref_count: 0.into(),
        };

        log::info!("derp");
        unsafe {
            let foo: *mut Foo = Box::into_raw(Box::from(foo));

            Foo::add_ref(foo as *mut cef_base_ref_counted_t);
            assert_eq!(Foo::has_one_ref(foo as *mut cef_base_ref_counted_t), 1);
            Foo::release(foo as *mut cef_base_ref_counted_t);
        }
    }
}
