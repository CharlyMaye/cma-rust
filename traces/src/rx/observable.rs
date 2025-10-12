//https://refactoring.guru/design-patterns/observer
use std::future::Future;
use std::thread;
use std::task::{RawWaker, RawWakerVTable, Waker, Context, Poll};
use std::ptr;

use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;

// TODO - on gère du unsafe nous même. Il est peut être préférable d'utiliser futures

// Unsubscribable indique si l'opération est déjà terminée (sync) ou si elle
// est lancée en arrière-plan (async). Le caller peut décider d'attendre via JoinHandle.
pub enum Unsubscribable {
    Ready,
    Background(std::thread::JoinHandle<()>),
}
impl Unsubscribable {}

pub trait Subscribable<TValue, TError> {
    // subscribe prend directement trois closures ; permet d'éviter Arc::new(...) côté appelant.
    fn subscribe<N, E, C>(
        &mut self,
        next: N,
        error: E,
        complete: C,
    ) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static;
}

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

impl<TValue, TError> Subscribable<TValue, TError> for Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    fn subscribe<N, E, C>(
        &mut self,
        next: N,
        error: E,
        complete: C,
    ) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static,
    {
        // construire l'observer en interne (pas d'Arc::new nécessaire côté appelant)
        let callbacks = Observer {
            next: std::sync::Arc::new(next),
            error: std::sync::Arc::new(error),
            complete: std::sync::Arc::new(complete),
        };

        match &self.teardown {
            TeardownLogic::Sync(arc_f) => {
                let f = arc_f.clone();
                match (f)(&callbacks) {
                    Ok(()) => Unsubscribable::Ready,
                    Err(e) => {
                        (callbacks.error)(e);
                        Unsubscribable::Ready
                    }
                }
            }
            TeardownLogic::Async(arc_f) => {
                let f = arc_f.clone();
                // cloner deux fois : une future reçoit une copie de l'observer,
                // l'autre clone est utilisé pour propager l'erreur si la future échoue.
                let cb_for_fut = callbacks.clone();
                let cb_for_err = callbacks.clone();

                // obtenir la future concrète (boxée par TeardownLogic::from_async)
                let mut fut = (f)(cb_for_fut);

                // créer un RawWaker "noop" pour construire un Context nécessaire au poll
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
                    // data pointer is null because our vtable callbacks do not touch it
                    let raw = RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE);
                    // this conversion is unsafe per API; keep it localized here
                    unsafe { Waker::from_raw(raw) }
                }

                // spawn a thread that polls the future until ready (no external executor)
                let handle = thread::spawn(move || {
                    // build a Waker/context
                    let waker = create_noop_waker();
                    let mut cx = Context::from_waker(&waker);

                    // poll loop
                    loop {
                        match fut.as_mut().poll(&mut cx) {
                            Poll::Ready(Ok(())) => break,
                            Poll::Ready(Err(e)) => {
                                (cb_for_err.error)(e);
                                break;
                            }
                            Poll::Pending => {
                                // no real wake source; yield to allow other threads to run
                                std::thread::yield_now();
                            }
                        }
                    }

                    // thread exits -> future driven to completion
                });

                Unsubscribable::Background(handle)
            }
        }
    }
}
