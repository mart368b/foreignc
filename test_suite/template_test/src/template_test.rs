use super::*;

#[test]
fn get_res_test() -> TResult<()> {
    let _resource = get_package_dir()?;
    Ok(())
}


#[test]
fn render_test() -> TResult<()> {
    let resource = get_package_dir();
    Ok(())
}