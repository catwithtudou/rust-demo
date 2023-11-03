// #![feature(future_join)]
//
// use core::future::join;
use std::thread;

// OS 线程
fn get_two_sites() {
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// rust 异步


async fn get_two_sites_async() {
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");
    future_one.await;
    future_two.await;
}


async fn download_async(url: &str) {
    println!("{}", url)
}

fn download(site: &str) {
    println!("{}", site)
}


#[cfg(test)]
mod chapter_one_test {
    use super::*;

    #[test]
    fn test_get_two_sites() {
        get_two_sites();
    }

    #[test]
    fn test_get_two_sites_async() {
        let _ = async {
            get_two_sites_async().await;
        };
    }
}


