use super::*;

#[test]
fn get_res_test() {
    let resource = get_parsed_dir();
    assert_eq!(3, resource.functions.len());
    assert_eq!(0, resource.free_functions.len());
    assert_eq!(0, resource.structs.len());
}
