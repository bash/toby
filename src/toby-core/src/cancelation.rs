use futures::task::Task;
use futures::{task, Async, Poll, Stream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

///
/// Creates a [`CancellationToken`] with an accompanying [`CancellationTokenSource`]
///
/// [`CancellationToken`]: ./struct.CancelationToken.html
/// [`CancellationTokenSource`]: ./struct.CancellationTokenSource.html
pub fn cancelation_token() -> (CancelationToken, CancellationTokenSource) {
    let inner = Arc::new(CancelationTokenInner::new());

    (
        CancelationToken(inner.clone()),
        CancellationTokenSource(inner),
    )
}

///
/// Signals to a [`CancellationToken`] that it should be canceled.
///
/// [`CancellationToken`]: ./struct.CancelationToken.html
///
#[derive(Debug, Clone)]
pub struct CancellationTokenSource(Arc<CancelationTokenInner>);

///
/// Propagates notification that operations should be canceled.  
/// The `CancelationToken` is passed to the operation that should be made cancelable.
///
#[derive(Debug)]
pub struct CancelationToken(Arc<CancelationTokenInner>);

#[derive(Debug)]
struct CancelationTokenInner {
    canceled: AtomicBool,
    task: RwLock<Option<Task>>,
}

///
/// A stream created by [`CancelableStreamExt.cancelable`].
///
/// [`CancelableStreamExt.cancelable`]: ./trait.CancelableStreamExt.html#method.cancelable
///
#[derive(Debug)]
pub struct CancelableStream<S>
where
    S: Stream,
{
    inner: S,
    token: CancelationToken,
}

pub trait CancelableStreamExt: Stream {
    fn cancelable(self, token: CancelationToken) -> CancelableStream<Self>
    where
        Self: Sized,
    {
        CancelableStream { inner: self, token }
    }
}

impl<S> CancelableStreamExt for S where S: Stream {}

impl<S> Stream for CancelableStream<S>
where
    S: Stream,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.token.is_canceled() {
            return Ok(Async::Ready(None));
        }

        self.token.set_current_task();

        self.inner.poll()
    }
}

impl CancelationTokenInner {
    fn new() -> Self {
        Self {
            canceled: AtomicBool::new(false),
            task: RwLock::new(None),
        }
    }
}

impl CancelationToken {
    pub(crate) fn set_current_task(&self) {
        if self.0.task.read().unwrap().is_none() {
            *self.0.task.write().unwrap() = Some(task::current());
        }
    }

    pub(crate) fn is_canceled(&self) -> bool {
        self.0.canceled.load(Ordering::SeqCst)
    }
}

impl CancellationTokenSource {
    pub fn cancel(&self) {
        self.0.canceled.store(true, Ordering::SeqCst);

        if let Some(ref task) = *self.0.task.read().unwrap() {
            task.notify();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cancellation_works() {
        struct MockStream {
            next_value: Option<u8>,
        }

        impl Stream for MockStream {
            type Item = u8;
            type Error = ();

            fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
                Ok(match self.next_value.take() {
                    Some(value) => Async::Ready(Some(value)),
                    None => Async::NotReady,
                })
            }
        }

        let (token, source) = cancelation_token();
        let called = Arc::new(AtomicBool::new(false));

        let stream = MockStream {
            next_value: Some(1),
        };

        {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(500));

                source.cancel();
            });
        }

        {
            let called = called.clone();

            tokio::run(stream.cancelable(token).for_each(move |value| {
                assert_eq!(1, value);
                assert_eq!(false, called.load(Ordering::SeqCst));
                called.store(true, Ordering::SeqCst);

                Ok(())
            }));
        }

        assert_eq!(true, called.load(Ordering::SeqCst));
    }
}
