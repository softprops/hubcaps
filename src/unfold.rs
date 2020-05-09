use crate::errors::Error;
use crate::utils::next_link;
use crate::Github;
use futures::stream;
use futures::{StreamExt, TryStreamExt};
use hyperx::header::Link;
use reqwest::Url;
use serde::de::DeserializeOwned;

type ItemsFn<Source, T> = Box<dyn Fn(Source) -> Vec<T> + Send + Sync>;
type PageResult<Source> = Result<(Option<Link>, Source), Error>;

pub async fn unfold<Source, T>(
    github: Github,
    initial: PageResult<Source>,
    to_items: ItemsFn<Source, T>,
) -> crate::Stream<T>
where
    Source: DeserializeOwned + 'static + Send + Sync,
    T: 'static + Send + Sync,
{
    match initial {
        Ok((link, source)) => {
            let items = to_items(source);
            let state = (github, to_items, link);
            let first = stream::once(async { Ok::<_, Error>(items) });

            let others = stream::try_unfold(state, |(github, to_items, link)| async move {
                if let Some(url) = parse_next_link(link)? {
                    let uri = [url.path(), url.query().unwrap_or_default()].join("?");

                    let (link, source) = github.get_pages(&uri).await?;
                    let items = to_items(source);

                    return Ok(Some((items, (github, to_items, link))));
                }
                Ok(None)
            });

            Box::pin(
                first
                    .chain(others)
                    .map_ok(|list| stream::iter(list.into_iter().map(Ok)))
                    .try_flatten(),
            )
        }
        Err(e) => Box::pin(stream::once(async { Err(e) })),
    }
}

fn parse_next_link(link: Option<Link>) -> Result<Option<Url>, Error> {
    link.and_then(|l| next_link(&l))
        .map(|s| Url::parse(&s).map_err(Error::from))
        .transpose()
}
