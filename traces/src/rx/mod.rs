//https://refactoring.guru/design-patterns/observer

pub struct Observer<TValue, TError> {
    pub next: fn(TValue),
    pub error: fn(TError),
    pub complete: fn(),
}
pub type TeardownLogic<TValue, TError> = fn(&Observer<TValue, TError>) -> Result<(), TError>;
fn default_teardown<TValue, TError>(_ : &Observer<TValue, TError>) -> Result<(), TError> {
    Ok(())
}

trait Unsubscribable {
    fn unsubscribe(&self) -> ();
}
trait Subscribable<TValue, TError> {
    fn subscribe(&mut self, callbacks: Observer<TValue, TError>) -> ();
}

struct Observable<TValue, TError> {
    teardown: TeardownLogic<TValue, TError>,
}
impl<TValue, TError> Observable<TValue, TError> {
    fn new(teardown: Option<TeardownLogic<TValue, TError>>) -> Self {
        Observable {
            teardown: teardown.unwrap_or(default_teardown::<TValue, TError>),
        }
    }
}
impl<TValue, TError> Subscribable<TValue, TError> for Observable<TValue, TError> {
    fn subscribe(&mut self, callbacks: Observer<TValue, TError>) -> () {
        match (self.teardown)(&callbacks) {
            Ok(()) => (),
            Err(e) => (callbacks.error)(e)
        }
    }
}
// test

pub fn test_rx() {
    // teardown qui réussit — passe Some(...)
    let mut obs_ok = Observable::<String, String>::new(Some(|obs: &Observer<String, String>| {
        (obs.next)("Hello from Observable".to_string());
        (obs.complete)();
        Ok(())
    }));
    let observer = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };
    obs_ok.subscribe(observer);

    // pas de teardown fourni — utilise default_teardown
    let mut obs_default = Observable::<String, String>::new(None);
    let observer2 = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };
    obs_default.subscribe(observer2);

    // teardown qui échoue
    let mut obs_err = Observable::<String, String>::new(Some(|_obs: &Observer<String, String>| {
        println!("Teardown logic executed (err)");
        Err("something went wrong".to_string())
    }));
    let observer3 = Observer {
        next: |v: String| println!("Observer next: {}", v),
        error: |e: String| println!("Observer error: {}", e),
        complete: || println!("Observer complete"),
    };
    obs_err.subscribe(observer3);
}
