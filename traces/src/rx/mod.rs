//https://refactoring.guru/design-patterns/observer

struct Observer {
    pub next: Box<dyn Fn(String) + 'static>,
    pub error: Box<dyn Fn(String) + 'static>,
    pub complete: Box<dyn Fn() + 'static>,
}
pub type TeardownLogic = fn(&Observer) -> Result<(), String>;

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
    fn new(teardown: TeardownLogic) -> Self {
        Observable { teardown }
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
    // teardown qui réussit
    let mut obs_ok = Observable::new(|obs| {
        (obs.next)("Hello from Observable".to_string());
        (obs.complete)();
        Ok(())
    });
    let observer = Observer {
        next: Box::new(|v| println!("Observer next: {}", v)),
        error: Box::new(|e| println!("Observer error: {}", e)),
        complete: Box::new(|| println!("Observer complete")),
    };
    obs_ok.subscribe(observer);

    // teardown qui échoue
    let mut obs_err = Observable::new(|_obs| {
        println!("Teardown logic executed (err)");
        Err("something went wrong".to_string())
    });
    let observer2 = Observer {
        next: Box::new(|v| println!("Observer next: {}", v)),
        error: Box::new(|e| println!("Observer error: {}", e)),
        complete: Box::new(|| println!("Observer complete")),
    };
    obs_err.subscribe(observer2);
}
