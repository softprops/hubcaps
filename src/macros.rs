macro_rules! json {
    ($input:ident) => {{
        match serde_json::to_vec(&$input) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }};
}

macro_rules! json_lit {
    ($($json:tt)+) => {
        match serde_json::to_vec(&serde_json::json!($($json)+)) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    };
}
