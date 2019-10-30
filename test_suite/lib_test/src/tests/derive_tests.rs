#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::derive_lib::*;

#[cfg(test)]
mod impl_test {
    use super::*;

    #[test]
    fn boxed_struct_test() {
        unsafe {
            let s = new_boxed_struct();
            assert_eq!(0, get_boxed_struct(s));
            inc_boxed_struct(s);
            assert_eq!(1, get_boxed_struct(s));
            free_boxed_struct(s);
        }
    }

    #[test]
    fn boxed_struct_multiple_inc_test() {
        unsafe {
            let s = new_boxed_struct();
            assert_eq!(0, get_boxed_struct(s));
            
            inc_boxed_struct(s);
            inc_boxed_struct(s);
            inc_boxed_struct(s);
            inc_boxed_struct(s);

            assert_eq!(4, get_boxed_struct(s));
            free_boxed_struct(s);
        }
    }
}

#[cfg(test)]
mod serde_test {
    use super::*;

    #[test]
    fn serde_struct_test() {
        unsafe {
            let s = new_serde_struct();
            assert_cstr("{\"value\":[\"Hello\",\"World!\"]}", s);
            free_string(s);
        }
    }

    #[test]
    fn serde_mixed_struct_test() {
        unsafe {
            let s = new_serde_struct();
            assert_cstr("{\"value\":[\"Hello\",\"World!\"]}", s);
            let b = json_to_box(s);
            let ss = box_to_json(b);
            assert_cstr("{\"value\":[\"Hello\",\"World!\"]}", ss);

            free_string(s);
            free_serde_struct(b);
            free_string(ss);
        }
    }
}

#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    fn hello_world_test() {
        unsafe {
            let s = hello_world();
            assert_cstr("Hello World!", s);
            free_string(s);
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
            free_string(ok);

            let err = throw_err(true);
            assert!(err.is_null());
            let lerr = last_error();
            assert_cstr("Err", lerr);
            free_string(lerr);
        }
    }

    #[test]
    fn option_test() {
        unsafe {
            let some = return_option(true);
            assert!(!some.is_null());
            assert_cstr("Some", some);
            free_string(some);

            let none = return_option(false);
            assert!(none.is_null());
        }
    }
}