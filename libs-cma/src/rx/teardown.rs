//https://refactoring.guru/design-patterns/observer
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::rx::observer::Observer;

type AsyncTeardownFuture<TError> = Pin<Box<dyn Future<Output = Result<(), TError>> + Send>>;
type SyncTeardownResult<TError> = Result<(), TError>;

/// Type alias pour une fonction de teardown synchrone
type SyncTeardownFn<TValue, TError> =
    Arc<dyn Fn(&Observer<TValue, TError>) -> SyncTeardownResult<TError> + Send + Sync + 'static>;

/// Type alias pour une fonction de teardown asynchrone
type AsyncTeardownFn<TValue, TError> =
    Arc<dyn Fn(Observer<TValue, TError>) -> AsyncTeardownFuture<TError> + Send + Sync + 'static>;

pub enum TeardownLogic<TValue, TError> {
    /// Exécution synchrone : la closure prend `&Observer` et retourne un Result.
    Sync(SyncTeardownFn<TValue, TError>),

    /// Exécution asynchrone : la closure prend un Observer (par valeur) et retourne une Future.
    /// La future est boxée et devra être conduite par subscribe() (ici on la drive dans un thread).
    Async(AsyncTeardownFn<TValue, TError>),
}

impl<TValue, TError> TeardownLogic<TValue, TError> {
    pub fn from_sync<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        TeardownLogic::Sync(Arc::new(f))
    }

    pub fn from_async<F, Fut>(f: F) -> Self
    where
        F: Fn(Observer<TValue, TError>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TError>> + Send + 'static,
    {
        let wrapper = move |obs: Observer<TValue, TError>| -> Pin<Box<dyn Future<Output = Result<(), TError>> + Send>> {
            Box::pin(f(obs))
        };
        TeardownLogic::Async(Arc::new(wrapper))
    }
}
