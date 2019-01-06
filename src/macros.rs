macro_rules! json {
    ($input:ident) => {
        serde_json::to_vec(&$input).expect("serialising data to vec")
    };
}

macro_rules! json_lit {
    ($($json:tt)+) => {
        // ideally:
        //     json!(serde_json::json!($($json)+))
        // but I can't get that to compile...
        //    error: no rules expected the token `::`
        //       --> src/macros.rs:9:25
        //        |
        //    9   |         json!(serde_json::json!($($json)+))
        //        |                         ^^
        serde_json::to_vec(&serde_json::json!($($json)+)).expect("serialising json literal to vec")
    };
}
