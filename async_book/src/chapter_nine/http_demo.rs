use std::fs;
// use std::io::prelude::*;
use std::time::Duration;
use async_std::task;
use async_std::prelude::*;
// use async_std::net::TcpStream;
// use futures::stream::StreamExt;
use std::marker::Unpin;
use async_std::io::{Read, Write};

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
// async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/chapter_nine/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(3)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "src/chapter_nine/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/chapter_nine/404.html")
    };

    let connects = fs::read_to_string(filename).unwrap();

    // 将响应写回流并刷新(flush)以确保响应被发送回客户端
    let response = format!("{}{}", status_line, connects);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[cfg(test)]
mod http_demo_test {
    use super::*;
    // use async_std::net::TcpListener;
    // use async_std::task::spawn;
    use futures::io::Error;
    use futures::task::{Context, Poll};

    use std::cmp::min;
    use std::pin::Pin;
    use std::marker::Unpin;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(self: Pin<&mut Self>,
                     _: &mut Context,
                     buf: &mut [u8]) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(mut self: Pin<&mut Self>,
                      _: &mut Context,
                      buf: &[u8]) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockTcpStream {}


    #[async_std::test]
    async fn test_handle_connection_test() {
        // let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
        //
        // listener
        //     .incoming().
        //     for_each_concurrent(None, |tcp_stream| async move {
        //         let tcp_stream = tcp_stream.unwrap();
        //         spawn(handle_connection(tcp_stream))
        //     }).await;

        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //     handle_connection(stream).await;
        // }

        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = fs::read_to_string("src/chapter_nine/hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}