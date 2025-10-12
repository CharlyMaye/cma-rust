//https://refactoring.guru/design-patterns/observer

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;


// Observer reste générique
pub struct Observer<TValue, TError> {
    pub next: fn(TValue),
    pub error: fn(TError),
    pub complete: fn(),
}
impl<TValue, TError> Clone for Observer<TValue, TError> {
    fn clone(&self) -> Self {
        Self {
            next: self.next,
            error: self.error,
            complete: self.complete,
        }
    }
}

// Teardown renvoie un Future boxed (pour la voie async)
pub type TeardownFuture<TError> =
    Pin<Box<dyn Future<Output = Result<(), TError>> + Send + 'static>>;

// TeardownLogic peut être sync ou async ; on stocke les closures dans Arc pour pouvoir
// cloner et déplacer dans un futur sans emprunts portant sur `self`.
#[derive(Clone)]
pub enum TeardownLogic<TValue: 'static, TError: 'static> {
    // Sync prend une référence à l'Observer — évite de déplacer `callbacks` pour le chemin sync
    Sync(Arc<dyn Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static>),
    // Async prend l'Observer par valeur (sera déplacé dans la future)
    Async(Arc<dyn Fn(Observer<TValue, TError>) -> TeardownFuture<TError> + Send + Sync + 'static>),
}
impl<TValue: 'static, TError: 'static> TeardownLogic<TValue, TError> {
    fn from_async<F, Fut>(f: F) -> Self
    where
        F: Fn(Observer<TValue, TError>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TError>> + Send + 'static,
    {
        TeardownLogic::Async(Arc::new(move |obs| Box::pin(f(obs))))
    }

    fn from_sync<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        TeardownLogic::Sync(Arc::new(f))
    }
}

// Unsubscribable indique si l'opération est déjà terminée (sync) ou si il y a
// un future à exécuter (async). L'appelant peut décider d'attendre le future ou non.
pub enum Unsubscribable {
    Ready,
    Pending(Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
}
impl Unsubscribable {
}

pub trait Subscribable<TValue, TError> {
    fn subscribe(
        &mut self,
        callbacks: Observer<TValue, TError>,
    ) -> Unsubscribable;
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
    fn subscribe(
        &mut self,
        callbacks: Observer<TValue, TError>,
    ) -> Unsubscribable {
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
