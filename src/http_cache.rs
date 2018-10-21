//! Implements <https://tools.ietf.org/html/rfc7232> Conditional Requests

use std;
use std::env;
use std::io;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use hyper::Uri;

use {Error, Result};

#[derive(Clone, Debug)]
pub enum HttpCache {
    Noop,
    FileBased(PathBuf),
}

use HttpCache::*;

impl HttpCache {
    pub fn noop() -> Self {
        HttpCache::Noop
    }

    pub fn in_home_dir() -> Self {
        #[allow(deprecated)] // TODO: Switch to the dirs crate.
        let mut dir = env::home_dir().expect("Expected a home dir");
        dir.push(".hubcaps/cache");
        HttpCache::FileBased(dir)
    }

    #[doc(hidden)]
    pub fn cache_body_and_etag(&self, uri: &str, body: &[u8], etag: &[u8]) -> Result<()> {
        match self {
            Noop => Ok(()),
            FileBased(dir) => {
                let mut path = cache_path(dir, &uri, "json");
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&path, body)?;
                path.set_extension("etag");
                fs::write(&path, etag)?;
                Ok(())
            },
        }
    }

    #[doc(hidden)]
    pub fn lookup_etag(&self, uri: &str) -> Result<String> {
        match self {
            Noop => HttpCache::no_read("No etag cached"),
            FileBased(dir) => read_to_string(cache_path(dir, uri, "etag")),
        }
    }

    #[doc(hidden)]
    pub fn lookup_body(&self, uri: &str) -> Result<String> {
        match self {
            Noop => HttpCache::no_read("No body cached"),
            FileBased(dir) => read_to_string(cache_path(dir, uri, "json")),
        }
    }

    fn no_read<E: Into<Box<std::error::Error + Send + Sync>>>(error: E) -> Result<String> {
        Err(Error::from(io::Error::new(io::ErrorKind::NotFound, error)))
    }
}

///       cache_path("https://api.github.com/users/dwijnand/repos", "json") ==>
/// ~/.hubcaps/cache/v1/https/api.github.com/users/dwijnand/repos.json
fn cache_path<S: AsRef<OsStr>>(dir: &Path, uri: &str, extension: S) -> PathBuf {
    let uri = uri.parse::<Uri>().expect("Expected a URI");
    let mut path = dir.to_path_buf();
    path.push("v1");
    path.push(uri.scheme_part().expect("Expected a URI scheme").as_ref()); // https
    path.push(uri.authority_part().expect("Expected a URI authority").as_ref()); // api.github.com
    path.push(
        Path::new(uri.path()) // /users/dwijnand/repos
            .strip_prefix("/")
            .expect("Expected URI path to start with /"),
    );
    path.set_extension(extension);
    path
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(path).map_err(Error::from)
}
