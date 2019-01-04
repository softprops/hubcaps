//! Activity interface

use hyper::client::connect::Connect;

use crate::notifications::Notifications;
use crate::stars::Stars;
use crate::watching::Watching;
use crate::Github;

pub struct Activity<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Activity<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    /// Return a reference to notifications operations
    pub fn notifications(&self) -> Notifications<C> {
        Notifications::new(self.github.clone())
    }

    /// return a reference to starring operations
    pub fn stars(&self) -> Stars<C> {
        Stars::new(self.github.clone())
    }

    /// Return a reference to watching operations
    /// https://developer.github.com/v3/activity/watching
    pub fn watching(&self) -> Watching<C> {
        Watching::new(self.github.clone())
    }
}
