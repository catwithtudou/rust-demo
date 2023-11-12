use futures::{future, future::FutureExt, pin_mut, select, Stream};
use futures::future::Fuse;
use futures::future::FusedFuture;
use futures::stream::{FusedStream, FuturesUnordered, StreamExt};

async fn task_one() { /* ... */ }

async fn task_two() { /* ... */ }

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(),
        }
        ;
    }

    println!("{}", total);
    assert_eq!(total, 10);
}
#[allow(dead_code)]

async fn add_two_streams(
    mut s1: impl Stream<Item=u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item=u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}
#[allow(dead_code)]
async fn get_new_num() -> u8 { /* ... */ 5 }
#[allow(dead_code)]
async fn run_on_new_num(_: u8) { /* ... */ }
#[allow(dead_code)]
async fn run_loop(
    mut interval_timer: impl Stream<Item=()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {},
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}


async fn run_on_new_num_push(_: u8) -> u8 { /* ... */ 5 }

#[allow(dead_code)]
async fn run_loop_future_unordered(
    mut interval_timer: impl Stream<Item=()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num_push(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_futs.push(run_on_new_num_push(new_num));
            },
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

#[cfg(test)]
mod select_test {
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn test_race_tasks() {
        block_on(race_tasks())
    }

    #[test]
    fn test_count() {
        block_on(count())
    }
}
