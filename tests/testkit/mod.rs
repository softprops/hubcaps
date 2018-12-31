use std::cell::Cell;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

fn global_test_root() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop(); // chop off exe name
    path.pop(); // chop off 'debug'

    // If `cargo test` is run manually then our path looks like
    // `target/debug/foo`, in which case our `path` is already pointing at
    // `target`. If, however, `cargo test --target $target` is used then the
    // output is `target/$target/debug/foo`, so our path is pointing at
    // `target/$target`. Here we conditionally pop the `$target` name.
    if path.file_name().and_then(|s| s.to_str()) != Some("target") {
        path.pop();
    }

    path.join("int-test")
}

fn test_root() -> PathBuf {
    let root = global_test_root();

    static NEXT_TEST_NUM: AtomicUsize = ATOMIC_USIZE_INIT;
    thread_local!(static TEST_NUM: usize = NEXT_TEST_NUM.fetch_add(1, Ordering::SeqCst));
    let root = root.join(&TEST_NUM.with(|my_id| format!("t{}", my_id)));

    thread_local!(static TEST_ROOT_INIT: Cell<bool> = Cell::new(false));
    TEST_ROOT_INIT.with(|i| {
        if i.get() {
            return;
        }
        i.set(true);
        fs::remove_dir_all(&root).expect("removing root");
        debug!("deleted root {}", root.display());
    });

    root
}

pub fn test_home() -> PathBuf {
    test_root().join("home")
}
