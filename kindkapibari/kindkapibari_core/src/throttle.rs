// use futures_core::Stream;
// use std::{
//     pin::Pin,
//     task::{Context, Poll},
//     time::Duration,
// };
// use tokio::{spawn, time::sleep};

// pub struct ThrottledBytes {
//     data: Vec<u8>,
//     per_100ms_cycle: usize,
//     rate: usize,
// }

// impl ThrottledBytes {
//     pub fn new(data: Vec<u8>, rate: usize) -> Self {
        
//     }
// }

// impl Stream for ThrottledBytes {
//     type Item = Vec<u8>;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         let to_send = match self.data.pop() {
//             Some(data) => data,
//             None => return Poll::Ready(None),
//         };

//         let waker = cx.waker().clone();
//         spawn(async || {
//             sleep(Duration::from_millis(100));
//             waker.wake();
//         });
//         Poll::Ready(Some(to_send))
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, Some(self.data.len()))
//     }
// }
