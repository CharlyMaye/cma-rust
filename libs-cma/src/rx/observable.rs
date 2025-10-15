//https://refactoring.guru/design-patterns/observer
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;

use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;

// TODO - on gère du unsafe nous même. Il est peut être préférable d'utiliser futures

// Unsubscribable indique si l'opération est déjà terminée (sync) ou si elle
// est lancée en arrière-plan (async). On utilise une struct contenant Option<JoinHandle>
// pour pouvoir prendre le handle (Option::take) sans déplacer un champ d'un type qui
// implémente Drop (évite l'erreur "cannot move out of type ... which implements Drop").
pub struct Unsubscribable {
    handle: Option<std::thread::JoinHandle<()>>,
    active: Arc<AtomicBool>,
}

impl Unsubscribable {
    pub fn ready() -> Self {
        Unsubscribable {
            handle: None,
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn background(handle: std::thread::JoinHandle<()>, active: Arc<AtomicBool>) -> Self {
        Unsubscribable {
            handle: Some(handle),
            active,
        }
    }

    /// Stop further callbacks immediately (non-blocking). The background thread may still run;
    /// callbacks are ignored because `active` is set to false.
    pub fn unsubscribe(&mut self) {
        self.active.store(false, Ordering::SeqCst);
    }

    /// Stop callbacks and wait for background task to finish (blocks).
    pub fn unsubscribe_and_wait(&mut self) -> std::thread::Result<()> {
        self.active.store(false, Ordering::SeqCst);
        self.join()
    }

    /// Wait for background task to finish (if any).
    pub fn join(&mut self) -> std::thread::Result<()> {
        if let Some(h) = self.handle.take() {
            h.join()
        } else {
            Ok(())
        }
    }

    /// Detach: join in a spawned thread so this call is non-blocking.
    pub fn detach(&mut self) {
        if let Some(h) = self.handle.take() {
            std::thread::spawn(move || {
                let _ = h.join();
            });
        }
    }
}

impl Drop for Unsubscribable {
    fn drop(&mut self) {
        // ensure callbacks are disabled on drop
        self.active.store(false, Ordering::SeqCst);
        if let Some(h) = self.handle.take() {
            // don't block Drop: detach the join
            std::thread::spawn(move || {
                let _ = h.join();
            });
        }
    }
}


#[derive(Debug)]
pub struct Observable<TValue: 'static, TError: 'static> {
    teardown: TeardownLogic<TValue, TError>,
}
impl<TValue: 'static, TError: 'static> Observable<TValue, TError> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        Observable {
            teardown: TeardownLogic::from_sync(f),
        }
    }

    /// with_async_teardown accepte maintenant une closure async (retourne une Future).
    /// subscribe() lancera et conduira cette future dans un thread.
    pub fn with_async_teardown<F, Fut>(f: F) -> Self
    where
        F: Fn(Observer<TValue, TError>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TError>> + Send + 'static,
    {
        Observable {
            teardown: TeardownLogic::from_async(f),
        }
    }
}

/// PIPE ///

impl<TValue, TError> Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    /// Opérateur map - transforme chaque valeur émise
    /// Ne s'exécute que lors du subscribe()
    pub fn map<U, F>(self, mapper: F) -> Observable<U, TError>
    where
        U: 'static + Send,
        F: Fn(TValue) -> U + Send + Sync + 'static,
    {
        let source_teardown = self.teardown;
        let mapper = Arc::new(mapper);

        // On crée un nouvel Observable qui "enregistre" la transformation
        match source_teardown {
            TeardownLogic::Sync(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_sync(move |observer: &Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);
                        let observer_next = Arc::clone(&observer.next);
                        let observer_error = Arc::clone(&observer.error);
                        let observer_complete = Arc::clone(&observer.complete);
                        let observer_active = Arc::clone(&observer.active);

                        // Observer intermédiaire qui applique map
                        let inner_observer = Observer {
                            next: Arc::new(move |value: TValue| {
                                let mapped = mapper(value);
                                observer_next(mapped);
                            }),
                            error: observer_error,
                            complete: observer_complete,
                            active: observer_active,
                        };

                        // Subscribe à la source
                        source_fn(&inner_observer)
                    }),
                }
            }
            TeardownLogic::Async(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_async(move |observer: Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);
                        let observer_next = Arc::clone(&observer.next);
                        let observer_error = Arc::clone(&observer.error);
                        let observer_complete = Arc::clone(&observer.complete);
                        let observer_active = Arc::clone(&observer.active);

                        // Observer intermédiaire qui applique map
                        let inner_observer = Observer {
                            next: Arc::new(move |value: TValue| {
                                let mapped = mapper(value);
                                observer_next(mapped);
                            }),
                            error: observer_error,
                            complete: observer_complete,
                            active: observer_active,
                        };

                        // Subscribe à la source (async)
                        source_fn(inner_observer)
                    }),
                }
            }
        }
    }


}

/// SUBSCRIBE ///
pub trait Subscribable<TValue, TError> {
    // subscribe prend directement trois closures ; permet d'éviter Arc::new(...) côté appelant.
    fn subscribe<N, E, C>(&mut self, next: N, error: E, complete: C) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static;
}
impl<TValue, TError> Subscribable<TValue, TError> for Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    fn subscribe<N, E, C>(&mut self, next: N, error: E, complete: C) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static,
    {
        // les Observer/wrappers (avec `active`) sont créés dans chaque branche (Sync/Async)
        // on n'instancie pas `callbacks` ici pour éviter d'oublier le champ `active`

        match &self.teardown {
            TeardownLogic::Sync(arc_f) => {
                let f = arc_f.clone();
                // create active flag true for sync as well (but will be used only if needed)
                let active = Arc::new(AtomicBool::new(true));
                let active_next = Arc::clone(&active);
                let active_err = Arc::clone(&active);
                let active_complete = Arc::clone(&active);

                let user_next = std::sync::Arc::new(next);
                let user_err = std::sync::Arc::new(error);
                let user_complete = std::sync::Arc::new(complete);

                let callbacks = Observer {
                    next: {
                        let u = Arc::clone(&user_next);
                        Arc::new(move |v| {
                            if active_next.load(Ordering::SeqCst) {
                                (u)(v)
                            }
                        })
                    },
                    error: {
                        let u = Arc::clone(&user_err);
                        Arc::new(move |e| {
                            if active_err.load(Ordering::SeqCst) {
                                (u)(e)
                            }
                        })
                    },
                    complete: {
                        let u = Arc::clone(&user_complete);
                        Arc::new(move || {
                            if active_complete.load(Ordering::SeqCst) {
                                (u)()
                            }
                        })
                    },
                    active: Arc::clone(&active),
                };

                match (f)(&callbacks) {
                    Ok(()) => Unsubscribable::ready(),
                    Err(e) => {
                        (callbacks.error)(e);
                        Unsubscribable::ready()
                    }
                }
            }
            TeardownLogic::Async(arc_f) => {
                let f = arc_f.clone();

                // prepare active flag shared between observer wrappers and Unsubscribable
                let active = Arc::new(AtomicBool::new(true));
                let active_next = Arc::clone(&active);
                let active_err = Arc::clone(&active);
                let active_complete = Arc::clone(&active);

                let user_next = std::sync::Arc::new(next);
                let user_err = std::sync::Arc::new(error);
                let user_complete = std::sync::Arc::new(complete);

                // Observer whose callbacks check `active` before invoking user closures
                let cb_for_fut = Observer {
                    next: {
                        let u = Arc::clone(&user_next);
                        Arc::new(move |v| {
                            if active_next.load(Ordering::SeqCst) {
                                (u)(v)
                            }
                        })
                    },
                    error: {
                        let u = Arc::clone(&user_err);
                        Arc::new(move |e| {
                            if active_err.load(Ordering::SeqCst) {
                                (u)(e)
                            }
                        })
                    },
                    complete: {
                        let u = Arc::clone(&user_complete);
                        Arc::new(move || {
                            if active_complete.load(Ordering::SeqCst) {
                                (u)()
                            }
                        })
                    },
                    active: Arc::clone(&active),
                };

                // second clone to be used only for error propagation by the driver
                let cb_for_err = cb_for_fut.clone();

                // obtenir la future concrète (boxée par TeardownLogic::from_async)
                let fut = (f)(cb_for_fut);

                // spawn the background driver and return its handle
                let handle = spawn_driven_future(fut, cb_for_err);

                Unsubscribable::background(handle, active)
            }
        }
    }
}


// no-op RawWaker callbacks (they must have unsafe fn signatures)
unsafe fn noop_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &RAW_WAKER_VTABLE)
}
unsafe fn noop_wake(_data: *const ()) {}
unsafe fn noop_wake_by_ref(_data: *const ()) {}
unsafe fn noop_drop(_data: *const ()) {}

static RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(noop_clone, noop_wake, noop_wake_by_ref, noop_drop);

// helper that confines the single unsafe needed to create a Waker
fn create_noop_waker() -> Waker {
    let raw = RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw) }
}

/// Drive a boxed future to completion on a background thread, using a noop waker.
/// Returns the JoinHandle so the caller can wait if desired.
fn spawn_driven_future<TValue, TError>(
    mut fut: Pin<Box<dyn Future<Output = Result<(), TError>> + Send>>,
    cb_for_err: Observer<TValue, TError>,
) -> std::thread::JoinHandle<()>
where
    TError: Send + 'static,
    TValue: 'static,
    Observer<TValue, TError>: Send + 'static,
{
    thread::spawn(move || {
        let waker = create_noop_waker();
        let mut cx = Context::from_waker(&waker);

        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(Ok(())) => break,
                Poll::Ready(Err(e)) => {
                    (cb_for_err.error)(e);
                    break;
                }
                Poll::Pending => {
                    std::thread::yield_now();
                }
            }
        }
    })
}
