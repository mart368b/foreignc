use crate::{Format, OutputFile};

struct PyFile{}

impl Format for PyFile {
    fn get_file_name(package_name: &str) -> String {
        format!("{}.py", package_name)
    }
    fn get_template() -> String {
        "This is my python script"
    }
    fn load_function(func) {
        
    }
}

impl OutputFile<PyFile> {

}
