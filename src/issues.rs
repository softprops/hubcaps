//! Issues interface

use self::super::{Github, Result, SortDirection, State};
use rep::{Issue, IssueReq, Label};
use rustc_serialize::json;
use std::fmt;
use std::default::Default;
use url::form_urlencoded;

#[derive(Debug, PartialEq)]
pub enum Filter {
    Assigned,
    Created,
    Mentioned,
    Subscribed,
    All,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Filter::Assigned => "assigned",
                   Filter::Created => "created",
                   Filter::Mentioned => "mentioned",
                   Filter::Subscribed => "subscribed",
                   Filter::All => "all",
               })
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::Assigned
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Comments,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Sort::Created => "created",
                   Sort::Updated => "updated",
                   Sort::Comments => "comments",
               })
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

pub struct IssueLabels<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueLabels<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> IssueLabels<'a>
        where O: Into<String>,
              R: Into<String>
    {
        IssueLabels {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues/{}/labels{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    /// add a set of labels to this issue ref
    pub fn add(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github
            .post::<Vec<Label>>(&self.path(""), try!(json::encode(&labels)).as_bytes())
    }

    /// remove a label from this issue
    pub fn remove(&self, label: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", label)))
    }

    /// replace all labels associated with this issue with a new set.
    /// providing an empty set of labels is the same as clearing the
    /// current labels
    pub fn set(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github.put::<Vec<Label>>(&self.path(""), try!(json::encode(&labels)).as_bytes())
    }

    /// remove all labels from an issue
    pub fn clear(&self) -> Result<()> {
        self.github
            .delete(&self.path(""))
    }
}

pub struct IssueRef<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueRef<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> IssueRef<'a>
        where O: Into<String>,
              R: Into<String>
    {
        IssueRef {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues/{}{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    pub fn labels(&self) -> IssueLabels {
        IssueLabels::new(self.github,
                         self.owner.as_ref(),
                         self.repo.as_ref(),
                         self.number)
    }

    pub fn edit(&self, is: &IssueReq) -> Result<Issue> {
        let data = try!(json::encode(&is));
        self.github.patch::<Issue>(&self.path(""), data.as_bytes())
    }
}


pub struct Issues<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

pub struct ListReq {
    state: State,
    sort: Sort,
    direction: SortDirection,
    assignee: Option<String>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Vec<String>,
    since: Option<String>,
}

impl ListReq {
    pub fn builder() -> ListReqBuilder {
        ListReqBuilder::new()
    }

    pub fn serialize(&self) -> String {
        let mut params = Vec::new();
        params.push(("state", self.state.to_string()));
        params.push(("sort", self.sort.to_string()));
        params.push(("direction", self.direction.to_string()));
        if let Some(ref a) = self.assignee {
            params.push(("assignee", a.to_owned()));
        }
        if let Some(ref c) = self.creator {
            params.push(("creator", c.to_owned()));
        }
        if let Some(ref m) = self.mentioned {
            params.push(("mentioned", m.to_owned()));
        }
        if let Some(ref s) = self.since {
            params.push(("since", s.to_owned()));
        }
        if !self.labels.is_empty() {
            params.push(("labels", self.labels.connect(",")));
        }
        form_urlencoded::serialize(params)
    }
}

/// an mutable issue list builder
#[derive(Default)]
pub struct ListReqBuilder {
    state: State,
    sort: Sort,
    direction: SortDirection,
    assignee: Option<String>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Vec<String>,
    since: Option<String>,
}

impl ListReqBuilder {
    pub fn new() -> ListReqBuilder {
        ListReqBuilder { ..Default::default() }
    }

    pub fn state(&mut self, state: State) -> &mut ListReqBuilder {
        self.state = state;
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut ListReqBuilder {
        self.sort = sort;
        self
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut ListReqBuilder {
        self.direction = direction;
        self
    }

    pub fn assignee<A>(&mut self, assignee: A) -> &mut ListReqBuilder
        where A: Into<String>
    {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn creator<C>(&mut self, creator: C) -> &mut ListReqBuilder
        where C: Into<String>
    {
        self.creator = Some(creator.into());
        self
    }

    pub fn mentioned<M>(&mut self, mentioned: M) -> &mut ListReqBuilder
        where M: Into<String>
    {
        self.mentioned = Some(mentioned.into());
        self
    }

    pub fn labels<L>(&mut self, labels: Vec<L>) -> &mut ListReqBuilder
        where L: Into<String>
    {
        self.labels = labels.into_iter().map(|l| l.into()).collect::<Vec<String>>();
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut ListReqBuilder
        where S: Into<String>
    {
        self.since = Some(since.into());
        self
    }

    pub fn build(&self) -> ListReq {
        ListReq {
            state: self.state.clone(),
            sort: self.sort.clone(),
            direction: self.direction.clone(),
            assignee: self.assignee.clone(),
            creator: self.creator.clone(),
            mentioned: self.mentioned.clone(),
            labels: self.labels.clone(),
            since: self.since.clone(),
        }
    }
}

impl<'a> Issues<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Issues<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Issues {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> IssueRef {
        IssueRef::new(self.github, self.owner.as_ref(), self.repo.as_ref(), number)
    }

    pub fn create(&self, is: &IssueReq) -> Result<Issue> {
        let data = try!(json::encode(&is));
        self.github.post::<Issue>(&self.path(""), data.as_bytes())
    }

    pub fn list(&self, req: &ListReq) -> Result<Vec<Issue>> {
        let url = self.path(&format!("?{}", req.serialize()));
        self.github.get::<Vec<Issue>>(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn list_reqs() {
        fn test_serialize(tests: Vec<(ListReq, &str)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![
            (
                ListReq::builder().build(),
                "state=open&sort=created&direction=asc"
            ),
            (
                ListReq::builder().state(State::Closed).build(),
                "state=closed&sort=created&direction=asc"
             ),
            (
                ListReq::builder().labels(vec!["foo", "bar"]).build(),
                "state=open&sort=created&direction=asc&labels=foo%2Cbar"
            ),
        ];
        test_serialize(tests)
    }

    #[test]
    fn filter_default() {
        let default: Filter = Default::default();
        assert_eq!(default, Filter::Assigned)
    }

    #[test]
    fn filter_display() {
        for (k, v) in vec![
            (Filter::Assigned, "assigned"),
            (Filter::Created, "created"),
            (Filter::Mentioned, "mentioned"),
            (Filter::Subscribed, "subscribed"),
            (Filter::All, "all"),
        ] {
            assert_eq!(k.to_string(), v)
        }
    }

    #[test]
    fn sort_default() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }

    #[test]
    fn sort_display() {
        for (k, v) in vec![(Sort::Created, "created"),
                           (Sort::Updated, "updated"),
                           (Sort::Comments, "comments")] {
            assert_eq!(k.to_string(), v)
        }
    }
}
