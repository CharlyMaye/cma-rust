//https://refactoring.guru/design-patterns/observer
mod block;
mod observer;
mod observable;
mod teardown;

use block::block_on;

// bring Subscribable (and Unsubscribable) into scope so `.subscribe()` is available
use observer::{Observer};
use observable::{Observable, Subscribable, Unsubscribable};

use std::sync::{Arc, Mutex};

// test (exemple d'usage)
// NOTE: pour exécuter le test dans un contexte synchrone, j'utilise futures::executor::block_on.
// Ajoute dans Cargo.toml : futures = "0.3"
pub fn test_rx() {
    test_new_observable();    
    test_return_value();
}

fn test_return_value() {
    let mut obs = Observable::<String, String>::new(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable (sync)".to_string());
        (obs.complete)();
        Ok(())
    });
    // use Arc<Mutex<...>> for shared mutable state accessible from the closure
    let value = Arc::new(Mutex::new(String::new()));
    let value_cloned = value.clone();

    // appeler subscribe directement avec les closures (plus de Observer::new)
    match obs.subscribe(
        move |v: String| {
            println!("Observer next: {}", v);
            let mut guard = value_cloned.lock().unwrap();
            *guard = v;
        },
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    };
}
fn test_new_observable() {
    let mut obs_ok = Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        (obs.next)("Hello from Observable (async)".to_string());
        (obs.complete)();
        Ok(())
    });

    match obs_ok.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }

    let mut obs_default = Observable::<String, String>::new(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable (sync)".to_string());
        (obs.complete)();
        Ok(())
    });
    match obs_default.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }

    let mut obs_err = Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        println!("Async teardown logic executed (err)");
        (obs.error)("something went wrong".to_string());
        Err("something went wrong".to_string())
    });
    match obs_err.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }
}