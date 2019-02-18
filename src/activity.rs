//! Activity interface
use crate::notifications::Notifications;
use crate::stars::Stars;
use crate::watching::Watching;
use crate::Github;

pub struct Activity {
    github: Github,
}

impl Activity {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    /// Return a reference to notifications operations
    pub fn notifications(&self) -> Notifications {
        Notifications::new(self.github.clone())
    }

    /// return a reference to starring operations
    pub fn stars(&self) -> Stars {
        Stars::new(self.github.clone())
    }

    /// Return a reference to watching operations
    /// https://developer.github.com/v3/activity/watching
    pub fn watching(&self) -> Watching {
        Watching::new(self.github.clone())
    }
}
