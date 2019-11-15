use foreignc::*;
use std::path::Path;

#[test]
fn get_res_test() -> TResult<()> {
    let _resource = get_package_dir()?;
    _resource.generate_python_api(Path::new("api.py"), None).unwrap();
    Ok(())
}


#[test]
fn render_test() -> TResult<()> {
    let _resource = get_package_dir();
    Ok(())
}