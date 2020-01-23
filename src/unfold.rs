use crate::errors::Error;
use crate::Github;
use futures::future::{BoxFuture, FutureExt};
use futures::stream;
use futures::Stream;
use hyperx::header::{Link, RelationType};
use reqwest::Url;
use serde::de::DeserializeOwned;

pub async fn unfold<Source, StreamOk>(
    github: Github,
    initial: Result<(Option<Link>, Source), Error>,
    to_items: Box<dyn Fn(Source) -> Vec<StreamOk> + Send + Sync>,
) -> crate::Stream<StreamOk>
where
    StreamOk: 'static + Send + Sync,
    Source: DeserializeOwned + 'static + Send + Sync,
{
    let state = StreamState::new(github, initial, to_items);

    Box::pin(stream::unfold(state, move |state| {
        async { state.next().await }
    }))
}

struct StreamState<Source, StreamOk>
where
    StreamOk: 'static + Send + Sync,
    Source: DeserializeOwned + 'static + Send + Sync,
{
    github: Github,
    items: Option<Result<Vec<StreamOk>, Error>>,
    to_items: Box<dyn Fn(Source) -> Vec<StreamOk> + Send + Sync>,
    next_page: Option<Link>,
}

impl<Source, StreamOk> StreamState<Source, StreamOk>
where
    StreamOk: 'static + Send + Sync,
    Source: DeserializeOwned + 'static + Send + Sync,
{
    fn new(
        github: Github,
        initial: Result<(Option<Link>, Source), Error>,
        to_items: Box<dyn Fn(Source) -> Vec<StreamOk> + Send + Sync>,
    ) -> Self {
        let dummy = Self {
            github,
            to_items,
            items: None,
            next_page: None,
        };

        dummy.load_state(initial)
    }

    async fn next(self) -> Option<(Result<StreamOk, Error>, Self)> {
        let mut state = self;
        loop {
            match state.flat_next() {
                FetcherState::NextReady(item, state) => {
                    return Some((item, state));
                }
                FetcherState::Empty => {
                    return None;
                }
                FetcherState::FetchNextPage(next_state) => {
                    state = next_state.fetch_next_page().await;
                }
            }
        }
    }

    fn flat_next(mut self) -> FetcherState<StreamOk, Self> {
        if let Some(resultitems) = self.items.take() {
            match resultitems {
                Ok(mut items) => {
                    if let Some(item) = items.pop() {
                        FetcherState::NextReady(
                            Ok(item),
                            Self {
                                github: self.github,
                                to_items: self.to_items,
                                items: Some(Ok(items)),
                                next_page: self.next_page,
                            },
                        )
                    } else if self.next_page.is_some() {
                        FetcherState::FetchNextPage(self)
                    } else {
                        // No more things, no more next pages
                        FetcherState::Empty
                    }
                }
                Err(e) => {
                    FetcherState::NextReady(
                        Err(e),
                        Self {
                            github: self.github,
                            to_items: self.to_items,
                            items: None,     // Returned once, just now
                            next_page: None, // Cannot have a next_page if Items is Err !!! fix the data model
                        },
                    )
                }
            }
        } else {
            if self.next_page.is_some() {
                FetcherState::FetchNextPage(self)
            } else {
                // No more things, no more next pages
                FetcherState::Empty
            }
        }
    }

    async fn fetch_next_page(mut self) -> Self {
        if let Some(next_page) = self.next_page.take() {
            if let Some(url) = next_link(&next_page) {
                if let Ok(url) = Url::parse(&url) {
                    let uri = [url.path(), url.query().unwrap_or_default()].join("?");

                    let fetched_page = self.github.get_pages(&uri).await;
                    return self.load_state(fetched_page);
                }
            }

            Self {
                github: self.github,
                items: self.items,
                to_items: self.to_items,
                next_page: None, // don't visit this path again,
                                 // !!! convert next_page at intake?
            }
        } else {
            self
        }
    }

    fn load_state(self, state: Result<(Option<Link>, Source), Error>) -> Self {
        let items: Result<Vec<StreamOk>, Error>;
        let next_page: Option<Link>;

        match state {
            Ok((link, payload)) => {
                items = Ok((self.to_items)(payload));
                next_page = link;
            }
            Err(e) => {
                items = Err(e);
                next_page = None;
            }
        }

        Self {
            github: self.github,
            to_items: self.to_items,
            items: Some(items),
            next_page,
        }
    }
}

enum FetcherState<StreamOk, State> {
    NextReady(Result<StreamOk, Error>, State),
    FetchNextPage(State),
    Empty,
}

pub fn next_link(l: &Link) -> Option<String> {
    l.values()
        .into_iter()
        .find(|v| v.rel().unwrap_or(&[]).get(0) == Some(&RelationType::Next))
        .map(|v| v.link().to_owned())
}
