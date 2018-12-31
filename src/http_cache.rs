//! Implements <https://tools.ietf.org/html/rfc7232> Conditional Requests

use std;
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

use dirs;
use hyper::Uri;

use {Error, Result};

pub type BoxedHttpCache = Box<HttpCache + Send>;

pub trait HttpCache: HttpCacheClone + Debug {
    fn cache_response(
        &self,
        uri: &str,
        body: &[u8],
        etag: &[u8],
        next_link: &Option<String>,
    ) -> Result<()>;
    fn lookup_etag(&self, uri: &str) -> Result<String>;
    fn lookup_body(&self, uri: &str) -> Result<String>;
    fn lookup_next_link(&self, uri: &str) -> Result<Option<String>>;
}

impl HttpCache {
    pub fn noop() -> BoxedHttpCache {
        Box::new(NoCache)
    }

    pub fn in_home_dir() -> BoxedHttpCache {
        let mut dir = dirs::home_dir().expect("Expected a home dir");
        dir.push(".hubcaps/cache");
        Box::new(FileBasedCache::new(dir))
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
    fn cache_response(&self, _: &str, _: &[u8], _: &[u8], _: &Option<String>) -> Result<()> {
        Ok(())
    }
    fn lookup_etag(&self, _uri: &str) -> Result<String> {
        no_read("No etag cached")
    }
    fn lookup_body(&self, _uri: &str) -> Result<String> {
        no_read("No body cached")
    }
    fn lookup_next_link(&self, _uri: &str) -> Result<Option<String>> {
        no_read("No next link cached")
    }
}

#[derive(Clone, Debug)]
pub struct FileBasedCache {
    root: PathBuf,
}

impl FileBasedCache {
    #[doc(hidden)] // public for integration testing only
    pub fn new<P: Into<PathBuf>>(root: P) -> FileBasedCache {
        FileBasedCache { root: root.into() }
    }
}

impl HttpCache for FileBasedCache {
    fn cache_response(
        &self,
        uri: &str,
        body: &[u8],
        etag: &[u8],
        next_link: &Option<String>,
    ) -> Result<()> {
        let mut path = cache_path(&self.root, &uri, "json");
        trace!("caching body at path: {}", path.display());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, body)?;
        path.set_extension("etag");
        fs::write(&path, etag)?;
        if let Some(next_link) = next_link {
            path.set_extension("next_link");
            fs::write(&path, next_link)?;
        }
        Ok(())
    }

    fn lookup_etag(&self, uri: &str) -> Result<String> {
        read_to_string(cache_path(&self.root, uri, "etag"))
    }

    fn lookup_body(&self, uri: &str) -> Result<String> {
        read_to_string(cache_path(&self.root, uri, "json"))
    }

    fn lookup_next_link(&self, uri: &str) -> Result<Option<String>> {
        let path = cache_path(&self.root, uri, "next_link");
        if path.exists() {
            Ok(Some(read_to_string(path)?))
        } else {
            Ok(None)
        }
    }
}

/// Construct the cache path for the given URI and extension, from an initial directory.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use hubcaps::http_cache::cache_path;
/// assert_eq!(
///     cache_path(&PathBuf::from("/home/.hubcaps/cache"), "https://api.github.com/users/dwijnand/repos", "json"),
///     PathBuf::from("/home/.hubcaps/cache/v1/https/api.github.com/users/dwijnand/repos.json"),
/// );
/// assert_eq!(
///     cache_path(&PathBuf::from("/home/.hubcaps/cache"), "https://api.github.com/users/dwijnand/repos?page=2", "json"),
///     PathBuf::from("/home/.hubcaps/cache/v1/https/api.github.com/users/dwijnand/repos/6dd58bde8abb0869.json"),
/// );
/// assert_eq!(
///     cache_path(&PathBuf::from("/home/.hubcaps/cache"), "https://api.github.com/users/dwijnand/repos?page=2&per_page=5", "json"),
///     PathBuf::from("/home/.hubcaps/cache/v1/https/api.github.com/users/dwijnand/repos/d862dcd2d85cebca.json"),
/// );
/// ```
#[doc(hidden)] // public for doc testing only
pub fn cache_path<S: AsRef<OsStr>>(dir: &Path, uri: &str, extension: S) -> PathBuf {
    let uri = uri.parse::<Uri>().expect("Expected a URI");
    let mut path = dir.to_path_buf();
    path.push("v1");
    path.push(uri.scheme_part().expect("no URI scheme").as_str()); // https
    path.push(uri.authority_part().expect("no URI authority").as_str()); // api.github.com
    path.push(Path::new(&uri.path()[1..])); // users/dwijnand/repos
    if let Some(query) = uri.query() {
        path.push(hash1(query, DefaultHasher::new())); // fa269019d5035d5f
    }
    path.set_extension(extension); // .json
    path
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    trace!("reading path: {}", path.as_ref().display());
    fs::read_to_string(path).map_err(Error::from)
}

fn no_read<T, E: Into<Box<std::error::Error + Send + Sync>>>(error: E) -> Result<T> {
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

fn hash1<A: Hash, H: Hasher>(x: A, mut hasher: H) -> String {
    x.hash(&mut hasher);
    u64_to_padded_hex(hasher.finish())
}

/// Construct a 0-padded hex string from a u64.
///
/// # Examples
///
/// ```
/// # use hubcaps::http_cache::u64_to_padded_hex;
/// assert_eq!(u64_to_padded_hex(0), "0000000000000000");
/// assert_eq!(u64_to_padded_hex(u64::max_value()), "ffffffffffffffff");
/// ```
#[doc(hidden)] // public for doc testing only
pub fn u64_to_padded_hex(x: u64) -> String {
    format!("{:016x}", x)
}
