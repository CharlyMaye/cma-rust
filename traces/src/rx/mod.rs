//https://refactoring.guru/design-patterns/observer
mod block;
mod observer;
mod observable;
mod teardown;

// bring Subscribable (and Unsubscribable) into scope so `.subscribe()` is available
use observer::{Observer};
use observable::{Observable, Subscribable, Unsubscribable};

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;

// test (exemple d'usage)
// NOTE: pour exécuter le test dans un contexte synchrone, j'utilise futures::executor::block_on.
// Ajoute dans Cargo.toml : futures = "0.3"
pub fn test_rx() {
    test_new_observable();    
    test_return_value();
    test_return_value_with_channel();
}

fn test_return_value() {
    let mut obs = create_sync_observable();
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
        Unsubscribable::Background(handle) => {
            // attendre explicitement si besoin
            let _ = handle.join();
        }
    };
}

fn test_return_value_with_channel() {
    let mut obs = create_sync_observable();

    // channel pour récupérer la valeur envoyée par le handler
    let (tx, rx) = mpsc::channel::<String>();

    match obs.subscribe(
        move |v: String| {
            // envoi via le channel ; ok pour usage sync ou async (si async, on attend plus bas)
            let _ = tx.send(v);
        },
        |e: String| {
            eprintln!("Observer error (channel): {}", e);
        },
        || println!("Observer complete (channel)"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Background(handle) => {
            // la future est démarrée par subscribe ; on peut attendre si on veut.
            let _ = handle.join();
        }
    };

    // On attend au maximum 1s pour éviter blocage indéfini
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(v) => println!("received via channel: {}", v),
        Err(e) => eprintln!("no value received from channel: {}", e),
    }
}

fn test_new_observable() {
    let mut obs_ok = create_async_observable();
    match obs_ok.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Background(handle) => {
            let _ = handle.join();
        }
    };
    let mut obs_default = create_sync_observable();
    match obs_default.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    ) {
        Unsubscribable::Ready => {}
        Unsubscribable::Background(handle) => {
            let _ = handle.join();
        }
    }
}

fn create_sync_observable() -> Observable::<String, String>{
    Observable::<String, String>::new(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable (sync)".to_string());
        (obs.complete)();
        Ok(())
    })
}
fn create_async_observable() -> Observable::<String, String>{
    Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        (obs.next)("Hello from Observable (async)".to_string());
        (obs.complete)();
        Ok(())
    })
}