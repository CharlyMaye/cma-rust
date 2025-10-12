//https://refactoring.guru/design-patterns/observer
use std::future::Future;
use std::pin::Pin;

use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;


// Unsubscribable indique si l'opération est déjà terminée (sync) ou si il y a
// un future à exécuter (async). L'appelant peut décider d'attendre le future ou non.
pub enum Unsubscribable {
    Ready,
    Pending(Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
}
impl Unsubscribable {
}

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
                let cb_for_call = callbacks.clone();
                let cb_for_err = callbacks.clone();
                let fut = Box::pin(async move {
                    match (f)(cb_for_call).await {
                        Ok(()) => (),
                        Err(e) => (cb_for_err.error)(e),
                    }
                }) as Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
                Unsubscribable::Pending(fut)
            }
        }
    }
}
