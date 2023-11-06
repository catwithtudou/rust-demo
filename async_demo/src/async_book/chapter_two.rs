trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct Socket {}

impl Socket {
    fn has_data_to_read(&mut self) {}
    fn read_buf(&mut self) -> Vec<u8> { Vec::new() }
}


pub struct SocketRead<'a> {
    socket: &'a Socket,
}


impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output=()>,
        FutureB: SimpleFuture<Output=()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output=()>,
        FutureB: SimpleFuture<Output=()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }
        self.second.poll(wake)
    }
}
