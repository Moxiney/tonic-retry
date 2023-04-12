#![feature(type_alias_impl_trait)]

use std::future::Future;

pub struct RpcRetryWrapper<C, I, F> {
    pub f: F,
    marker: std::marker::PhantomData<fn(&mut C, I)>,
}

impl<C, I, F> RpcRetryWrapper<C, I, F> {
    fn new(f: F) -> Self {
        Self {
            f,
            marker: std::marker::PhantomData,
        }
    }
}

impl<C, I, O, F> RpcRetryWrapper<C, I, F>
where
    I: Clone,
    F: for<'a> Fn(&'a mut C, I) -> LifetimeBoundFutureWrapper<'a, O::Output, O>,
    O: Future,
{
    async fn call_once(&self, client: &mut C, request: I) -> O::Output {
        (self.f)(client, request).fut.await
    }

    async fn call_twice(&self, client: &mut C, request: I) -> O::Output {
        let first_res = (self.f)(client, request.clone()).fut.await;
        let second_res = (self.f)(client, request.clone()).fut.await;
        second_res
    }
}

trait LifetimeBoundFuture<'a, T>: 'a + Future<Output = T> {}

impl<'a, T, F> LifetimeBoundFuture<'a, T> for F where F: 'a + Future<Output = T> {}

struct LifetimeBoundFutureWrapper<'a, T, F> {
    fut: F,
    marker: std::marker::PhantomData<fn() -> &'a T>,
}

impl<'a, T, F> From<F> for LifetimeBoundFutureWrapper<'a, T, F>
where
    F: LifetimeBoundFuture<'a, T>,
{
    fn from(fut: F) -> Self {
        Self {
            fut,
            marker: std::marker::PhantomData,
        }
    }
}

/// This function produce a future with lifetime 'a.
/// call_example should statisfy the trait bound:
/// Future: LifetimeBoundFuture<'a, O>
/// Self: for<'a> Fn(&'a mut C, I) -> Future
async fn call_exmaple(client: &mut String, request: String) -> String {
    request
}

#[cfg(test)]
mod tests {
    use crate::{call_exmaple, LifetimeBoundFutureWrapper};

    #[tokio::test]
    async fn future_wrapper_test() {
        let mut client = "client".to_string();
        let request = "hello".to_string();

        let fut_wrapper: LifetimeBoundFutureWrapper<'_, String, _> =
            call_exmaple(&mut client, request).into();
    }
}
