//https://refactoring.guru/design-patterns/observer
mod block;
mod observer;

use block::block_on;

// bring Subscribable (and Unsubscribable) into scope so `.subscribe()` is available
use observer::{Observable, Observer, Subscribable, Unsubscribable};

// test (exemple d'usage)
// NOTE: pour exécuter le test dans un contexte synchrone, j'utilise futures::executor::block_on.
// Ajoute dans Cargo.toml : futures = "0.3"
pub fn test_rx() {

    // teardown async qui réussit
    let mut obs_ok = Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        (obs.next)("Hello from Observable (async)".to_string());
        (obs.complete)();
        Ok(())
    });

    let observer = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };

    // subscribe retourne Unsubscribable — on attend le futur uniquement s'il existe
    match obs_ok.subscribe(observer.clone()) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }

    // teardown sync -> subscribe returns Ready (pas de futur)
    let mut obs_default = Observable::<String, String>::new(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable (sync)".to_string());
        (obs.complete)();
        Ok(())
    });
    let observer2 = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };
    match obs_default.subscribe(observer2) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }

    // teardown async qui échoue
    let mut obs_err = Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        println!("Async teardown logic executed (err)");
        (obs.error)("something went wrong".to_string());
        Err("something went wrong".to_string())
    });
    let observer3 = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };
    match obs_err.subscribe(observer3) {
        Unsubscribable::Ready => {}
        Unsubscribable::Pending(fut) => { block_on(fut); }
    }
}
