#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hello_world_test() {
        unsafe {
            let s = hello_world();
            assert_cstr("Hello World!", s);
        }
    }

    #[test]
    fn add_test() {
        unsafe {
            assert_eq!(2, add(1, 1));
        }
    }

    #[test]
    fn result_test() {
        unsafe {
            let ok = throw_err(false);
            assert!(!ok.is_null());
            assert_cstr("Ok", ok);

            let err = throw_err(true);
            assert!(err.is_null());
            let lerr = last_error();
            assert_cstr("Err", lerr);
        }
    }

    #[test]
    fn option_test() {
        unsafe {
            let some = return_option(true);
            assert!(!some.is_null());
            assert_cstr("Some", some);

            let none = return_option(false);
            assert!(none.is_null());
        }
    }
}