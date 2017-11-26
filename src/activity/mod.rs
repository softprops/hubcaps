//! Activity interface

use hyper::client::Connect;

use Github;
use stars::Stars;

pub struct Activity<C>
where
    C: Clone + Connect,
{
    github: Github<C>,
}

impl<C: Clone + Connect> Activity<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }
    pub fn stars(&self) -> Stars<C> {
        Stars::new(self.github.clone())
    }
}
