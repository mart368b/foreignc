#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn serde_struct_test() {
        unsafe {
            let s = new_serde_struct();
            assert_cstr("{\"value\":[\"Hello\",\"World!\"]}", s);
        }
    }
}