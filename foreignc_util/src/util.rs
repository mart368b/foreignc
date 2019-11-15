pub fn to_snake_case(s: String) -> String {
    let mut ss = String::new();
    let mut require_space = false;
    for c in s.chars() {
        if c.is_uppercase() && require_space {
            ss.push('_');
        }
        if c.is_lowercase() {
            require_space = true;
        } else if !c.is_alphabetic() && !c.is_numeric() {
            require_space = false;
        }
        ss.push(c.to_lowercase().next().unwrap());
    }

    ss
}

pub fn to_camel_case(s: String) -> String {
    let mut ss = String::new();
    let mut require_upper = true;
    for c in s.chars() {
        if c.is_whitespace() || c == '_' {
            require_upper = true;
        }else if require_upper {
            ss.push(c.to_uppercase().next().unwrap());
            require_upper = false;
        }else {
            ss.push(c);
        }
    }

    ss
}