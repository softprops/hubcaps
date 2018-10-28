macro_rules! json {
    ($input:ident) => {{
        match serde_json::to_vec(&$input) {
            Ok(data) => data,
            Err(err) => return Box::new(future::err(err.into())),
        }
    }};
}
