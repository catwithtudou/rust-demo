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
    println!("{}", url);
}

fn download(site: &str) {
    println!("{}", site);
}


async fn hello_world() {
    println!("hello_world");
}

async fn learn_song() {
    println!("learn_song");
}

async fn sing_song() {
    println!("sing_song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let _song = learn_song().await;
    sing_song().await;
}

async fn block_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1,f2);
}


#[cfg(test)]
mod simple_demo_test {
    use futures::executor::block_on;

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

    #[test]
    fn test_hello_world() {
        let future = hello_world();
        block_on(future);
    }

    #[test]
    fn test_block_main() {
        block_on(block_main());
    }
}


