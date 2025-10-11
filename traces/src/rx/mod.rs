//https://refactoring.guru/design-patterns/observer

struct Observer<'a> {
    pub next: Box<dyn Fn(String) + 'a>,
    pub error: Box<dyn Fn(String) + 'a>,
    pub complete: Box<dyn Fn() + 'a>,
}
pub type TeardownLogic = fn(&Observer) -> Result<(), String>;
fn default_teardown(_: &Observer) -> Result<(), String> {
    Ok(())
}

trait Unsubscribable {
    fn unsubscribe(&self) -> ();
}
trait Subscribable {
    fn subscribe(&mut self, callbacks: Observer) -> ();
}

struct Observable {
    teardown: TeardownLogic,
}
impl Observable {
    fn new(teardown: Option<TeardownLogic>) -> Self {
        Observable {
            teardown: teardown.unwrap_or(default_teardown),
        }
    }
}
impl Subscribable for Observable {
    fn subscribe(&mut self, callbacks: Observer) -> () {
        match (self.teardown)(&callbacks) {
            Ok(()) => (),
            Err(e) => (callbacks.error)(e)
        }
    }
}
// test

pub fn test_rx() {
    // teardown qui réussit — passe Some(...)
    let mut obs_ok = Observable::new(Some(|obs| {
        (obs.next)("Hello from Observable".to_string());
        (obs.complete)();
        Ok(())
    }));
    let observer = Observer {
        next: Box::new(|v| println!("Observer next: {}", v)),
        error: Box::new(|e| println!("Observer error: {}", e)),
        complete: Box::new(|| println!("Observer complete")),
    };
    obs_ok.subscribe(observer);

    // pas de teardown fourni — utilise default_teardown
    let mut obs_default = Observable::new(None);
    let observer2 = Observer {
        next: Box::new(|v| println!("Observer next: {}", v)),
        error: Box::new(|e| println!("Observer error: {}", e)),
        complete: Box::new(|| println!("Observer complete")),
    };
    obs_default.subscribe(observer2);

    // teardown qui échoue
    let mut obs_err = Observable::new(Some(|_obs| {
        println!("Teardown logic executed (err)");
        Err("something went wrong".to_string())
    }));
    let observer2 = Observer {
        next: Box::new(|v| println!("Observer next: {}", v)),
        error: Box::new(|e| println!("Observer error: {}", e)),
        complete: Box::new(|| println!("Observer complete")),
    };
    obs_err.subscribe(observer2);
}
