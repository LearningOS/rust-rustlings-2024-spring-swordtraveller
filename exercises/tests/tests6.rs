// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let mut data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let raw_ptr = Box::into_raw(data);
        let ret = unsafe { raw_pointer_to_box(raw_ptr) };

        unsafe {
            (*raw_ptr).b = Some("hello".to_owned());
        }

        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2, "The addresses of `a` should be the same.");
        assert_eq!(ret.b, Some("hello".to_owned()), "The `b` field should now be `Some(`hello`)`.");
    }
}
