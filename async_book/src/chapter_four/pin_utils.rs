// `pin_utils` 可以在crates.io中找到


#[cfg(test)]
mod pin_utils_test {
    use std::future::Future;

    use pin_utils::pin_mut;

    fn execute_unpin_future(x: impl Future<Output=()> + Unpin) { /* ... */ }
    // fn execute_unpin_future(x: Pin<Box<dyn Future<Output=()>>>) { /* ... */ }


    #[test]
    fn test_test() {
        // let fut = async { /* ... */ };
        // execute_unpin_future(fut); // Error: `fut` does not implement `Unpin` trait

        let fut = async { /* ... */ };
        let fut = Box::pin(fut);
        execute_unpin_future(fut); // OK

        let fut = async { /* ... */ };
        pin_mut!(fut);
        execute_unpin_future(fut); // OK
    }
}