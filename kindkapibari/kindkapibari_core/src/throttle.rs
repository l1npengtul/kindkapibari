use futures_core::Stream;
use poem::Body;
use redis::aio::tokio;
use std::pin::Pin;
use std::process::id;
use std::task::{Context, Poll};
use std::time::Duration;

pub struct ThrottledBytes {
    data: Vec<Vec<u8>>,
    per_100ms_cycle: usize,
    rate: usize,
}

impl ThrottledBytes {
    pub fn new(data: Vec<u8>, rate: usize) -> Self {
        if rate != 0 {
            let per_100ms_cycle = rate / 10;
            let mut data = data.chunks(chunks).collect();
            data.reverse();
            Self {
                data,
                per_100ms_cycle,
                rate,
            }
        } else {
            let mut all = Vec::new();
            all.push(data);
            Self {
                data: all,
                per_100ms_cycle,
                rate,
            }
        }
    }
}

impl Stream for ThrottledBytes {
    type Item = Vec<u8>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let to_send = match self.data.pop() {
            Some(data) => data,
            None => return Poll::Ready(None),
        };

        let waker = cx.waker().clone();
        tokio::task::spawn(async || {
            tokio::time::sleep(Duration::from_millis(100));
            waker.wake();
        });
        Poll::Ready(Some(to_send))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.data.len()))
    }
}

impl Into<Body> for ThrottledBytes {
    fn into(self) -> Body {
        Body::from_bytes_stream(self)
    }
}
