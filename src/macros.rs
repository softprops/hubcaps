macro_rules! json {
    ($input:ident) => {{
        match serde_json::to_vec(&$input) {
            Ok(data) => data,
            Err(err) => return Box::new(futures::future::err(err.into())),
        }
    }};
}

macro_rules! json_lit {
    ($($json:tt)+) => {
        match serde_json::to_vec(&serde_json::json!($($json)+)) {
            Ok(data) => data,
            Err(err) => return Box::new(futures::future::err(err.into())),
        }
    };
}
