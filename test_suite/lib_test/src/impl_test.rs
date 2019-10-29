use crate::*;

mod tests {
    use crate::*;

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
}