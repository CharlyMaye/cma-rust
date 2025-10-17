//! # Reactive Programming Module
//! 
//! This module provides reactive programming patterns inspired by RxJS for Rust.
//! It implements the Observer pattern with Observable streams, operators for data 
//! transformation, and subscription management.
//! 
//! Based on the Observer design pattern: https://refactoring.guru/design-patterns/observer
//! and https://github.com/ReactiveX/rxjs
//! ## Core Components
//! 
//! - `Observable`: Represents a stream of data that can be observed
//! - `Observer`: Handles next values, errors, and completion events
//! - `Subscription`: Manages the lifecycle and cancellation of subscriptions
//! - `Operators`: Transform and manipulate data streams (map, filter, etc.)

pub mod observable;
pub mod observer;
pub mod operators;
pub mod teardown;

/// Test function for the reactive programming system.
/// 
/// This is a placeholder function used during development to test
/// the rx module functionality.
pub fn test_rx() {}

#[cfg(test)]
mod tests {
    use super::observable::*;
    use super::observer::*;

    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    fn create_sync_observable() -> Observable<String, String> {
        Observable::<String, String>::new(|obs: &Observer<String, String>| {
            (obs.next)("Hello from Observable (sync)".to_string());
            (obs.complete)();
            Ok(())
        })
    }

    fn create_async_observable() -> Observable<String, String> {
        Observable::<String, String>::with_async_teardown(
            |obs: Observer<String, String>| async move {
                (obs.next)("Hello from Observable (async)".to_string());
                (obs.complete)();
                Ok(())
            },
        )
    }

    #[test]
    fn test_pipe_operators() {
        use std::sync::{Arc, Mutex};

        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);

        // Nothing executes here - we're just building the chain
        let mut observable: Observable<i32, ()> = Observable::new(|observer| {
            println!("Source executed!");
            (observer.next)(1);
            (observer.next)(2);
            (observer.next)(3);
            (observer.next)(4);
            (observer.next)(5);
            (observer.complete)();
            Ok(())
        })
        .map(|x| {
            println!("Map: {} -> {}", x, x * 2);
            x * 2
        });

        println!("Before subscribe - nothing has executed yet");

        // EVERYTHING executes now
        let mut sub = observable.subscribe(
            move |x| {
                println!("Received: {}", x);
                results_clone.lock().unwrap().push(x);
            },
            |e| eprintln!("Error: {:?}", e),
            || println!("Completed"),
        );

        sub.join().unwrap();

        let final_results = results.lock().unwrap();
        assert_eq!(*final_results, vec![2, 4, 6, 8, 10]);
    }
    #[test]
    fn test_sync_observable_with_mutex() {
        let mut obs = create_sync_observable();
        let value = Arc::new(Mutex::new(String::new()));
        let value_cloned = value.clone();

        let mut unsub = obs.subscribe(
            move |v: String| {
                let mut guard = value_cloned.lock().unwrap();
                *guard = v;
            },
            |e: String| eprintln!("Observer error: {}", e),
            || {},
        );

        let _ = unsub.join();

        let result = value.lock().unwrap();
        assert_eq!(*result, "Hello from Observable (sync)");
    }

    #[test]
    fn test_sync_observable_with_channel() {
        let mut obs = create_sync_observable();

        let (tx, rx) = mpsc::channel::<String>();
        let mut unsub = obs.subscribe(
            move |v: String| {
                let _ = tx.send(v);
            },
            |e: String| {
                eprintln!("Observer error (channel): {}", e);
            },
            || {},
        );

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(v) => {
                assert_eq!(v, "Hello from Observable (sync)");
            }
            Err(e) => panic!("no value received from channel: {}", e),
        }
        unsub.detach();
    }

    #[test]
    fn test_async_observable() {
        let mut obs = create_async_observable();
        let (tx, rx) = mpsc::channel::<String>();

        let mut unsub = obs.subscribe(
            move |v: String| {
                let _ = tx.send(v);
            },
            |e: String| eprintln!("Observer error: {}", e),
            || {},
        );

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(v) => {
                assert_eq!(v, "Hello from Observable (async)");
            }
            Err(e) => panic!("no value received: {}", e),
        }
        unsub.detach();
    }

    #[test]
    fn test_unsubscribe_stops_callbacks() {
        let mut obs = Observable::with_async_teardown(|obs: Observer<String, String>| async move {
            let mut i = 0;
            while obs.is_active() {
                (obs.next)(format!("msg {}", i));
                i += 1;
                std::thread::sleep(Duration::from_millis(50));
            }
            (obs.complete)();
            Ok(())
        });

        let (tx, rx) = mpsc::channel::<String>();

        let mut unsub = obs.subscribe(
            move |v: String| {
                let _ = tx.send(v);
            },
            |e: String| eprintln!("Observer error: {}", e),
            || {},
        );

        // receive the first message
        let first = rx.recv_timeout(Duration::from_secs(1));
        assert!(first.is_ok(), "Should receive first message");

        // stop callbacks
        unsub.unsubscribe();

        // verify we don't receive anything else
        let result = rx.recv_timeout(Duration::from_millis(200));
        assert!(
            result.is_err(),
            "Should not receive messages after unsubscribe"
        );
    }

    #[test]
    fn test_unsubscribe_and_wait() {
        let mut obs = Observable::with_async_teardown(|obs: Observer<String, String>| async move {
            let mut i = 0;
            while obs.is_active() {
                (obs.next)(format!("msg {}", i));
                i += 1;
                std::thread::sleep(Duration::from_millis(30));
            }
            (obs.complete)();
            Ok(())
        });

        let (tx, rx) = mpsc::channel::<String>();

        let mut unsub = obs.subscribe(
            move |v: String| {
                let _ = tx.send(v);
            },
            |e: String| eprintln!("Observer error: {}", e),
            || {},
        );

        // receive a message
        let first = rx.recv_timeout(Duration::from_secs(1));
        assert!(first.is_ok(), "Should receive message");

        // wait for task completion
        let result = unsub.unsubscribe_and_wait();
        assert!(result.is_ok(), "unsubscribe_and_wait should succeed");
    }

    #[test]
    fn test_multiple_subscribers() {
        let mut obs = create_sync_observable();

        let value1 = Arc::new(Mutex::new(String::new()));
        let value2 = Arc::new(Mutex::new(String::new()));

        let v1_clone = value1.clone();
        let v2_clone = value2.clone();

        let mut unsub1 = obs.subscribe(
            move |v: String| {
                *v1_clone.lock().unwrap() = v;
            },
            |_| {},
            || {},
        );

        let mut obs2 = create_sync_observable();
        let mut unsub2 = obs2.subscribe(
            move |v: String| {
                *v2_clone.lock().unwrap() = v;
            },
            |_| {},
            || {},
        );

        let _ = unsub1.join();
        let _ = unsub2.join();

        assert_eq!(*value1.lock().unwrap(), "Hello from Observable (sync)");
        assert_eq!(*value2.lock().unwrap(), "Hello from Observable (sync)");
    }
}
