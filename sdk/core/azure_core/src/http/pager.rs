// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{headers::HeaderName, response::Response};
use async_trait::async_trait;
use futures::{stream::unfold, FutureExt, Stream};
use std::{fmt, future::Future, pin::Pin, task};
use typespec::Error;
use typespec_client_core::http::{DeserializeWith, Format, JsonFormat};

/// The result of fetching a single page from a [`Pager`], whether there are more pages or paging is done.
pub enum PagerResult<P, N> {
    /// There are more pages the [`Pager`] may fetch using the `next` token.
    More { response: P, next: N },
    /// The [`Pager`] is done and there are no additional pages to fetch.
    Done { response: P },
}

impl<P, F> PagerResult<Response<P, F>, String> {
    /// Creates a [`PagerResult<P, C>`] from the provided response, extracting the continuation value from the provided header.
    ///
    /// If the provided response has a header with the matching name, this returns [`PagerResult::More`], using the value from the header as the continuation.
    /// If the provided response does not have a header with the matching name, this returns [`PagerResult::Done`].
    pub fn from_response_header(response: Response<P, F>, header_name: &HeaderName) -> Self {
        match response.headers().get_optional_string(header_name) {
            Some(next) => PagerResult::More { response, next },
            None => PagerResult::Done { response },
        }
    }
}

impl<P, N: fmt::Debug> fmt::Debug for PagerResult<P, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::More { next, .. } => f
                .debug_struct("More")
                .field("next", &next)
                .finish_non_exhaustive(),
            Self::Done { .. } => f.debug_struct("Done").finish_non_exhaustive(),
        }
    }
}

/// Represents a single page of items returned by a collection request to a service.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Page {
    /// The type of items in the collection.
    type Item;
    /// The type containing items in the collection e.g., [`Vec<Self::Item>`](Vec).
    type IntoIter: Iterator<Item = Self::Item>;

    /// Gets a single page of items returned by a collection request to a service.
    async fn into_items(self) -> crate::Result<Self::IntoIter>;
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<P, F> Page for Response<P, F>
where
    P: DeserializeWith<F> + Page + Send,
    F: Format + Send,
{
    type Item = P::Item;
    type IntoIter = P::IntoIter;
    async fn into_items(self) -> crate::Result<Self::IntoIter> {
        let page: P = self.into_body().await?;
        page.into_items().await
    }
}

/// Represents a paginated stream of items returned by a collection request to a service.
///
/// Specifically, this is a [`ItemIterator`] that yields [`Response<T>`] items.
pub type Pager<P, F = JsonFormat> = ItemIterator<Response<P, F>>;

#[cfg(not(target_arch = "wasm32"))]
type BoxedStream<P> = Box<dyn Stream<Item = Result<P, Error>> + Send>;

#[cfg(target_arch = "wasm32")]
type BoxedStream<P> = Box<dyn Stream<Item = Result<P, Error>>>;

/// Iterates over a collection of items or individual pages of items from a service.
///
/// You can asynchronously iterate over items returned by a collection request to a service,
/// or asynchronously fetch pages of items if preferred.
#[pin_project::pin_project]
pub struct ItemIterator<P: Page> {
    #[pin]
    stream: Pin<BoxedStream<P>>,
    current: Option<P::IntoIter>,
}

impl<P: Page> ItemIterator<P> {
    /// Creates a [`ItemIterator<P>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single `Option<N>` parameter, and returns a [`PagerResult<T, N>`] value asynchronously.
    /// The `N` type parameter is the type of the next link/token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](typespec::Result<T>) values.
    ///
    /// The first time your callback is called, it will be called with [`Option::None`], indicating no next link/token is present.
    ///
    /// Your callback must return one of:
    /// * `Ok(result)` - The request succeeded, and the provided [`PagerResult`] indicates the value to return and if there are more pages.
    /// * `Err(..)` - The request failed. The error will be yielded to the stream, the stream will end, and the callback will not be called again.
    ///
    /// ## Examples
    ///
    /// To page results using a next link:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, ItemIterator, Page, PagerResult, Pipeline, RawResponse, Request, Response, Method, Url}, json};
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    ///     next_link: Option<String>,
    /// }
    /// #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    /// #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    /// impl Page for ListItemsResult {
    ///     type Item = String;
    ///     type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    ///     async fn into_items(self) -> Result<Self::IntoIter> {
    ///         Ok(self.items.into_iter())
    ///     }
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = ItemIterator::from_callback(move |next_link: Option<Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let Some(next_link) = next_link {
    ///             // Ensure the api-version from the client is appended.
    ///             let qp = next_link
    ///                 .query_pairs()
    ///                 .filter(|(name, _)| name.ne("api-version"));
    ///             req
    ///                 .url_mut()
    ///                 .query_pairs_mut()
    ///                 .clear()
    ///                 .extend_pairs(qp)
    ///                 .append_pair("api-version", &api_version);
    ///         }
    ///         let resp = pipeline
    ///           .send(&Context::new(), &mut req)
    ///           .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let bytes = body.collect().await?;
    ///         let result: ListItemsResult = json::from_json(&bytes)?;
    ///         let resp: Response<ListItemsResult> = RawResponse::from_bytes(status, headers, bytes).into();
    ///         Ok(match result.next_link {
    ///             Some(next_link) => PagerResult::More {
    ///                 response: resp,
    ///                 next: next_link.parse()?,
    ///             },
    ///             None => PagerResult::Done { response: resp }
    ///         })
    ///     }
    /// });
    /// ```
    ///
    /// To page results using headers:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, ItemIterator, Page, PagerResult, Pipeline, Request, Response, Method, headers::HeaderName}};
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    /// }
    /// #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    /// #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    /// impl Page for ListItemsResult {
    ///     type Item = String;
    ///     type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    ///     async fn into_items(self) -> Result<Self::IntoIter> {
    ///         Ok(self.items.into_iter())
    ///     }
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = ItemIterator::from_callback(move |continuation| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let Some(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&Context::new(), &mut req)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-next-continuation")))
    ///     }
    /// });
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<N>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] N: 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(Option<N>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + 'static,
    >(
        make_request: F,
    ) -> Self {
        Self::from_stream(iter_from_callback(make_request))
    }

    /// Creates a [`ItemIterator<P>`] from a raw stream of [`Result<P>`](typespec::Result<P>) values.
    ///
    /// This constructor is used when you are implementing a completely custom stream and want to use it as a pager.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = Result<P, Error>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = Result<P, Error>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
            current: None,
        }
    }

    /// Gets a [`PageIterator<P>`] to iterate over a collection of pages from a service.
    ///
    /// You can use this to asynchronously iterate pages returned by a collection request to a service.
    /// This allows you to get the individual pages' [`Response<P>`], from which you can iterate items in each page
    /// or deserialize the raw response as appropriate.
    pub fn into_pages(self) -> PageIterator<P> {
        PageIterator {
            stream: self.stream,
        }
    }
}

impl<P: Page> futures::Stream for ItemIterator<P> {
    type Item = Result<P::Item, Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let mut projected_self = self.project();
        loop {
            if let Some(current) = projected_self.current.as_mut() {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }
                // Reset the iterator and poll for the next page.
                *projected_self.current = None;
            }

            match projected_self.stream.as_mut().poll_next(cx) {
                task::Poll::Ready(page) => match page {
                    Some(Ok(page)) => match page.into_items().poll_unpin(cx) {
                        task::Poll::Ready(Ok(iter)) => {
                            *projected_self.current = Some(iter);
                            continue;
                        }
                        task::Poll::Ready(Err(err)) => return task::Poll::Ready(Some(Err(err))),
                        task::Poll::Pending => return task::Poll::Pending,
                    },
                    Some(Err(err)) => return task::Poll::Ready(Some(Err(err))),
                    None => return task::Poll::Ready(None),
                },
                task::Poll::Pending => return task::Poll::Pending,
            }
        }
    }
}

impl<P: Page> fmt::Debug for ItemIterator<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemIterator").finish_non_exhaustive()
    }
}

/// Iterates over a collection pages of items from a service.
#[pin_project::pin_project]
pub struct PageIterator<P> {
    #[pin]
    stream: Pin<BoxedStream<P>>,
}

impl<P> PageIterator<P> {
    /// Creates a [`PageIterator<P>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single `Option<N>` parameter, and returns a [`PagerResult<T, N>`] value asynchronously.
    /// The `N` type parameter is the type of the next link/token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](typespec::Result<T>) values.
    ///
    /// The first time your callback is called, it will be called with [`Option::None`], indicating no next link/token is present.
    ///
    /// Your callback must return one of:
    /// * `Ok(result)` - The request succeeded, and the provided [`PagerResult`] indicates the value to return and if there are more pages.
    /// * `Err(..)` - The request failed. The error will be yielded to the stream, the stream will end, and the callback will not be called again.
    ///
    /// ## Examples
    ///
    /// To page results using a next link:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, PageIterator, PagerResult, Pipeline, RawResponse, Request, Response, Method, Url}, json};
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    ///     next_link: Option<String>,
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = PageIterator::from_callback(move |next_link: Option<Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let Some(next_link) = next_link {
    ///             // Ensure the api-version from the client is appended.
    ///             let qp = next_link
    ///                 .query_pairs()
    ///                 .filter(|(name, _)| name.ne("api-version"));
    ///             req
    ///                 .url_mut()
    ///                 .query_pairs_mut()
    ///                 .clear()
    ///                 .extend_pairs(qp)
    ///                 .append_pair("api-version", &api_version);
    ///         }
    ///         let resp = pipeline
    ///           .send(&Context::new(), &mut req)
    ///           .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let bytes = body.collect().await?;
    ///         let result: ListItemsResult = json::from_json(&bytes)?;
    ///         let resp: Response<ListItemsResult> = RawResponse::from_bytes(status, headers, bytes).into();
    ///         Ok(match result.next_link {
    ///             Some(next_link) => PagerResult::More {
    ///                 response: resp,
    ///                 next: next_link.parse()?,
    ///             },
    ///             None => PagerResult::Done { response: resp }
    ///         })
    ///     }
    /// });
    /// ```
    ///
    /// To page results using headers:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, PageIterator, PagerResult, Pipeline, Request, Response, Method, headers::HeaderName}};
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = PageIterator::from_callback(move |continuation| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let Some(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&Context::new(), &mut req)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-ms-continuation")))
    ///     }
    /// });
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<N>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] N: 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(Option<N>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + 'static,
    >(
        make_request: F,
    ) -> Self {
        Self::from_stream(iter_from_callback(make_request))
    }

    /// Creates a [`PageIterator<P>`] from a raw stream of [`Result<P>`](typespec::Result<P>) values.
    ///
    /// This constructor is used when you are implementing a completely custom stream and want to use it as a pager.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = Result<P, Error>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = Result<P, Error>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl<P> futures::Stream for PageIterator<P> {
    type Item = Result<P, Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}

impl<P> fmt::Debug for PageIterator<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageIterator").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State<T> {
    Init,
    More(T),
    Done,
}

fn iter_from_callback<
    P,
    // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
    #[cfg(not(target_arch = "wasm32"))] N: Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<N>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + Send + 'static,
    #[cfg(target_arch = "wasm32")] N: 'static,
    #[cfg(target_arch = "wasm32")] F: Fn(Option<N>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = Result<PagerResult<P, N>, typespec::Error>> + 'static,
>(
    make_request: F,
) -> impl Stream<Item = Result<P, Error>> + 'static {
    unfold(
        // We flow the `make_request` callback through the state value so that we can avoid cloning.
        (State::Init, make_request),
        |(state, make_request)| async move {
            let result = match state {
                State::Init => make_request(None).await,
                State::More(c) => make_request(Some(c)).await,
                State::Done => return None,
            };
            let (item, next_state) = match result {
                Err(e) => return Some((Err(e), (State::Done, make_request))),
                Ok(PagerResult::More {
                    response,
                    next: continuation,
                }) => (Ok(response), State::More(continuation)),
                Ok(PagerResult::Done { response }) => (Ok(response), State::Done),
            };

            // Flow 'make_request' through to avoid cloning
            Some((item, (next_state, make_request)))
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::http::{
        headers::{HeaderName, HeaderValue},
        Pager, PagerResult, RawResponse, StatusCode,
    };
    use async_trait::async_trait;
    use futures::{StreamExt as _, TryStreamExt as _};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Page {
        pub items: Vec<i32>,
        pub page: i32,
    }

    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    impl super::Page for Page {
        type Item = i32;
        type IntoIter = <Vec<i32> as IntoIterator>::IntoIter;

        async fn into_items(self) -> crate::Result<Self::IntoIter> {
            Ok(self.items.into_iter())
        }
    }

    #[tokio::test]
    async fn callback_item_pagination() {
        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                None => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"items":[1],"page":1}"#,
                    )
                    .into(),
                    next: "1",
                }),
                Some("1") => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-2"),
                        )])
                        .into(),
                        r#"{"items":[2],"page":2}"#,
                    )
                    .into(),
                    next: "2",
                }),
                Some("2") => Ok(PagerResult::Done {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-3"),
                        )])
                        .into(),
                        r#"{"items":[3],"page":3}"#,
                    )
                    .into(),
                }),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let items: Vec<i32> = pager.try_collect().await.unwrap();
        assert_eq!(vec![1, 2, 3], items.as_slice())
    }

    #[tokio::test]
    async fn callback_item_pagination_error() {
        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                None => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"items":[1],"page":1}"#,
                    )
                    .into(),
                    next: "1",
                }),
                Some("1") => Err(typespec::Error::message(
                    typespec::error::ErrorKind::Other,
                    "yon request didst fail",
                )),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let pages: Vec<Result<(String, Page), typespec::Error>> = pager
            .into_pages()
            .then(|r| async move {
                let r = r?;
                let header = r
                    .headers()
                    .get_optional_string(&HeaderName::from_static("x-test-header"))
                    .unwrap();
                let body = r.into_body().await?;
                Ok((header, body))
            })
            .collect()
            .await;
        assert_eq!(2, pages.len());
        assert_eq!(
            &(
                "page-1".to_string(),
                Page {
                    items: vec![1],
                    page: 1
                }
            ),
            pages[0].as_ref().unwrap()
        );

        let err = pages[1].as_ref().unwrap_err();
        assert_eq!(&typespec::error::ErrorKind::Other, err.kind());
        assert_eq!("yon request didst fail", format!("{}", err));
    }
}
