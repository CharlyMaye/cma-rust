//https://refactoring.guru/design-patterns/observer
mod observable;
mod observer;
mod teardown;

// bring Subscribable (and Unsubscribable) into scope so `.subscribe()` is available
use observable::{Observable, Subscribable};
use observer::Observer;

use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// test (exemple d'usage)
// NOTE: pour exécuter le test dans un contexte synchrone, j'utilise futures::executor::block_on.
// Ajoute dans Cargo.toml : futures = "0.3"
pub fn test_rx() {
    test_new_observable();
    test_return_value();
    test_return_value_with_channel();
    test_unsubscribe_examples();
}

fn test_return_value() {
    let mut obs = create_sync_observable();
    let value = Arc::new(Mutex::new(String::new()));
    let value_cloned = value.clone();

    let mut unsub = obs.subscribe(
        move |v: String| {
            println!("Observer next: {}", v);
            let mut guard = value_cloned.lock().unwrap();
            *guard = v;
        },
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    );

    let _ = unsub.join();
}

fn test_return_value_with_channel() {
    let mut obs = create_sync_observable();

    let (tx, rx) = mpsc::channel::<String>();
    let mut unsub = obs.subscribe(
        move |v: String| {
            let _ = tx.send(v);
        },
        |e: String| {
            eprintln!("Observer error (channel): {}", e);
        },
        || println!("Observer complete (channel)"),
    );

    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(v) => println!("received via channel: {}", v),
        Err(e) => eprintln!("no value received from channel: {}", e),
    }
    unsub.detach();
}

fn test_new_observable() {
    let mut obs_ok = create_async_observable();
    let mut unsub_ok = obs_ok.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    );
    unsub_ok.detach();

    let mut obs_default = create_sync_observable();
    let mut unsub_default = obs_default.subscribe(
        |v: String| println!("Observer next: {}", v),
        |e: String| println!("Observer error: {}", e),
        || println!("Observer complete"),
    );
    let _ = unsub_default.join();
}

fn test_unsubscribe_examples() {
    // exemple non‑test unitaire : démontre unsubscribe() et unsubscribe_and_wait()
    let mut obs = Observable::with_async_teardown(|mut obs: Observer<String, String>| async move {
        let mut i = 0;
        while obs.is_active() {
            println!("background loop (example) iteration {}", i);
            (obs.next)(format!("msg {}", i));
            i += 1;
            std::thread::sleep(Duration::from_millis(50));
        }
        (obs.complete)();
        Ok(())
    });

    // channel pour collecter les messages
    let (tx, rx) = mpsc::channel::<String>();

    // démarrer l'observation
    let mut unsub = obs.subscribe(
        move |v: String| {
            let _ = tx.send(v);
        },
        |e: String| eprintln!("Observer error (example): {}", e),
        || println!("Observer complete (example)"),
    );

    // recevoir le premier message
    if let Ok(v) = rx.recv_timeout(Duration::from_secs(1)) {
        println!("example received first: {}", v);
    }

    // arrêter immédiatement les callbacks sans attendre la fin du thread
    unsub.unsubscribe();
    // vérifier qu'on ne reçoit plus rien
    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(v) => println!("unexpected message after unsubscribe: {}", v),
        Err(_) => println!("no more messages after unsubscribe (expected)"),
    }

    // --- maintenant démonstration unsubscribe_and_wait ---
    let mut obs2 =
        Observable::with_async_teardown(|mut obs: Observer<String, String>| async move {
            let mut i = 0;
            while obs.is_active() {
                println!("background loop (example2) iteration {}", i);
                (obs.next)(format!("xmsg {}", i));
                i += 1;
                std::thread::sleep(Duration::from_millis(30));
            }
            (obs.complete)();
            Ok(())
        });

    let (tx2, rx2) = mpsc::channel::<String>();

    let mut unsub2 = obs2.subscribe(
        move |v: String| {
            let _ = tx2.send(v);
        },
        |e: String| eprintln!("Observer error (example2): {}", e),
        || println!("Observer complete (example2)"),
    );

    // recevoir un message
    let _ = rx2.recv_timeout(Duration::from_secs(1));

    // demander l'arrêt des callbacks et attendre la fin de la tâche en arrière‑plan
    let _ = unsub2.unsubscribe_and_wait();
    println!("unsubscribe_and_wait finished for example2");
}

fn create_sync_observable() -> Observable<String, String> {
    Observable::<String, String>::new(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable (sync)".to_string());
        (obs.complete)();
        Ok(())
    })
}
fn create_async_observable() -> Observable<String, String> {
    Observable::<String, String>::with_async_teardown(|obs: Observer<String, String>| async move {
        (obs.next)("Hello from Observable (async)".to_string());
        (obs.complete)();
        Ok(())
    })
}
