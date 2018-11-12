//! Implements <https://tools.ietf.org/html/rfc7232> Conditional Requests

use std;
use std::env;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use dirs;
use hyper::Uri;

use {Error, Result};

pub type BoxedHttpCache = Box<HttpCache + Send>;

pub trait HttpCache: HttpCacheClone + Debug {
    fn cache_body_and_etag(&self, uri: &str, body: &[u8], etag: &[u8]) -> Result<()>;
    fn lookup_etag(&self, uri: &str) -> Result<String>;
    fn lookup_body(&self, uri: &str) -> Result<String>;
}

impl HttpCache {
    pub fn noop() -> BoxedHttpCache {
        Box::new(NoCache)
    }

    pub fn in_home_dir() -> BoxedHttpCache {
        let mut dir = dirs::home_dir().expect("Expected a home dir");
        dir.push(".hubcaps/cache");
        Box::new(FileBasedCache { root: dir })
    }
}

impl Clone for BoxedHttpCache {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone, Debug)]
pub struct NoCache;

impl HttpCache for NoCache {
    fn cache_body_and_etag(&self, _: &str, _: &[u8], _: &[u8]) -> Result<()> {
        Ok(())
    }
    fn lookup_etag(&self, _uri: &str) -> Result<String> {
        no_read("No etag cached")
    }
    fn lookup_body(&self, _uri: &str) -> Result<String> {
        no_read("No body cached")
    }
}

#[derive(Clone, Debug)]
pub struct FileBasedCache {
    root: PathBuf,
}

impl HttpCache for FileBasedCache {
    fn cache_body_and_etag(&self, uri: &str, body: &[u8], etag: &[u8]) -> Result<()> {
        let mut path = cache_path(&self.root, &uri, "json");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, body)?;
        path.set_extension("etag");
        fs::write(&path, etag)?;
        Ok(())
    }

    fn lookup_etag(&self, uri: &str) -> Result<String> {
        read_to_string(cache_path(&self.root, uri, "etag"))
    }

    fn lookup_body(&self, uri: &str) -> Result<String> {
        read_to_string(cache_path(&self.root, uri, "json"))
    }
}

///       cache_path("https://api.github.com/users/dwijnand/repos", "json") ==>
/// ~/.hubcaps/cache/v1/https/api.github.com/users/dwijnand/repos.json
fn cache_path<S: AsRef<OsStr>>(dir: &Path, uri: &str, extension: S) -> PathBuf {
    let uri = uri.parse::<Uri>().expect("Expected a URI");
    let mut path = dir.to_path_buf();
    path.push("v1");
    path.push(uri.scheme_part().expect("Expected a URI scheme").as_ref()); // https
    path.push(
        uri.authority_part()
            .expect("Expected a URI authority")
            .as_ref(),
    ); // api.github.com
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

fn no_read<E: Into<Box<std::error::Error + Send + Sync>>>(error: E) -> Result<String> {
    Err(Error::from(io::Error::new(io::ErrorKind::NotFound, error)))
}

// Separate to provide a blanket implementation for `T: HttpCache + Clone`
// https://stackoverflow.com/a/30353928/463761
#[doc(hidden)]
pub trait HttpCacheClone {
    #[doc(hidden)]
    fn box_clone(&self) -> BoxedHttpCache;
}

impl<T> HttpCacheClone for T
where
    T: 'static + HttpCache + Clone + Send,
{
    fn box_clone(&self) -> BoxedHttpCache {
        Box::new(self.clone())
    }
}
