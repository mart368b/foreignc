#[allow(unused_imports)]
use crate::derive_lib_bind::*;
#[allow(unused_imports)]
use crate::*;

#[cfg(test)]
mod impl_test {
    use super::*;
    use foreignc::{CResult, CString};

    #[test]
    pub fn return_string_test(){
        unsafe {
            // Call function
            let ptr = return_string_ffi();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get value
            let msg_ptr = s.value as *mut *mut c_char;
            let msg = &mut **msg_ptr;
            assert_cstr("Hello World!", msg as *mut c_char);

            // Clean up
            free_string(msg as *mut c_char);
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn return_number_test(){
        unsafe {
            // Call function
            let ptr = return_number_ffi();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get value
            let msg_ptr = s.value as *mut u32;
            assert_eq!(12345, *msg_ptr);

            // Clean up
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn return_some_number_test(){
        unsafe {
            // Call function
            let ptr = return_some_number_ffi();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get option
            let opt_ptr = s.value as *mut *mut u32;
            // Get value
            let opt = *opt_ptr;
            assert!(!opt.is_null());
            assert_eq!(12345, *opt);

            // Clean up
            free_coption(*opt_ptr as *mut c_void);
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn return_none_number_test(){
        unsafe {
            // Call function
            let ptr = return_none_number_ffi();
            let s = &*ptr;
            assert!(!s.is_err);
            
            // Get option
            let opt_ptr = s.value as *mut *mut u32;
            // Get value
            let opt = *opt_ptr;
            assert!(opt.is_null());

            // Clean up
            free_coption(*opt_ptr as *mut c_void);
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn return_ok_number_test(){
        unsafe {
            // Call function
            let ptr = return_ok_number_ffi();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get result
            let res_ptr = s.value as *mut *mut CResult<u32, String>;
            let res = &**res_ptr;
            assert!(!res.is_err);

            // Get value
            let value_ptr = res.value as *mut u32;
            assert!(!value_ptr.is_null());
            assert_eq!(12345, *value_ptr);
            
            // Clean up
            free_cresult(*res_ptr as *mut c_void);
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn return_err_str_test(){
        unsafe {
            // Call function
            let ptr = return_err_str_ffi();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get result
            let res_ptr = s.value as *mut *mut CResult<u32, String>;
            let res = &**res_ptr;
            assert!(res.is_err);

            // Get value
            let value_ptr = res.value as *mut *mut c_char;
            assert!(!value_ptr.is_null());
            assert_cstr("Hello World!", *value_ptr);
            
            // Clean up
            free_string(*value_ptr as *mut c_char);
            free_cresult(*res_ptr as *mut c_void);
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn number_argument_test(){
        unsafe {
            // Get result
            let ptr = number_argument_ffi(12345);
            let s = &*ptr;
            assert!(!s.is_err);

            // Clean up
            free_cresult(ptr as *mut c_void);
        }
    }

    #[test]
    pub fn str_argument_test(){
        unsafe {
            // Create argument
            let msg = CString::new("Hello World!").unwrap();
            let msg_ptr = msg.into_raw();

            // Get result
            let ptr = str_argument_ffi(msg_ptr);
            let s = &*ptr;
            assert!(!s.is_err);

            // Clean up
            free_cresult(ptr as *mut c_void);
            CString::from_raw(msg_ptr);
        }
    }

    #[test]
    fn json_struct_test() {
        unsafe {
            // Get result
            let ptr = new_boxed_struct();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get BoxedStruct
            let boxed_ptr = s.value as *mut *mut c_void;
            assert!(!boxed_ptr.is_null());
            let boxed = *boxed_ptr;

            /////////////////////////
            // Check counter value //
            /////////////////////////
            let get0_ptr = get_boxed_struct(boxed);
            let s0 = &*get0_ptr;
            assert!(!s0.is_err);

            // Get value
            let get_ptr0 = s0.value as *mut u32;
            assert!(!get_ptr0.is_null());
            assert_eq!(0, *get_ptr0);

            ///////////////
            // Increment //
            ///////////////
            let inc0_ptr = inc_boxed_struct(boxed);
            let s1 = &*inc0_ptr;
            assert!(!s1.is_err);

            /////////////////////////
            // Check counter value //
            /////////////////////////
            let get1_ptr = get_boxed_struct(boxed);
            let s2 = &*get1_ptr;
            assert!(!s2.is_err);

            // Get value
            let get_ptr1 = s2.value as *mut u32;
            assert_eq!(1, *get_ptr1);

            //Clean up
            free_boxed_struct(boxed);
            free_cresult(ptr as *mut c_void);
            free_cresult(get0_ptr as *mut c_void);
            free_cresult(get1_ptr as *mut c_void);
            free_cresult(inc0_ptr as *mut c_void);
        }
    }

    #[test]
    fn box_struct_test() {
        unsafe {
            // Get result
            let ptr = new_json_struct();
            let s = &*ptr;
            assert!(!s.is_err);

            // Get JsonStruct V0
            let json_ptr = s.value as *mut *mut c_void;
            assert!(!json_ptr.is_null());
            let json0 = *json_ptr;

            /////////////////////////
            // Check counter value //
            /////////////////////////
            let get0_ptr = get_json_struct(json0);
            let s0 = &*get0_ptr;
            assert!(!s0.is_err);

            // Get value
            let get_ptr0 = s0.value as *mut u32;
            assert!(!get_ptr0.is_null());
            assert_eq!(0, *get_ptr0);

            ///////////////
            // Increment //
            ///////////////
            let inc0_ptr = inc_json_struct(json0);
            let s1 = &*inc0_ptr;
            assert!(!s1.is_err);

            // Get JsonStruct V1
            let inc_ptr0 = s1.value as *mut *mut c_void;
            assert!(!inc_ptr0.is_null());
            let json1 = *inc_ptr0;

            /////////////////////////
            // Check counter value //
            /////////////////////////
            let get1_ptr = get_json_struct(json1);
            let s2 = &*get1_ptr;
            assert!(!s2.is_err);

            // Get value
            let get_ptr1 = s2.value as *mut u32;
            assert_eq!(1, *get_ptr1);

            //Clean up
            free_string(json0 as *mut c_char);
            free_string(json1 as *mut c_char);
            free_cresult(ptr as *mut c_void);
            free_cresult(get0_ptr as *mut c_void);
            free_cresult(get1_ptr as *mut c_void);
            free_cresult(inc0_ptr as *mut c_void);
        }
    }
}
