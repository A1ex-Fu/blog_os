use core::{future::Future, pin::Pin};
use alloc::boxed::Box;

pub mod simple_executor;


pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}


use core::task::{Context, Poll};

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}





