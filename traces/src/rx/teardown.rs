//https://refactoring.guru/design-patterns/observer
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

use crate::rx::observer::Observer;

pub enum TeardownLogic<TValue, TError> {
    /// Exécution synchrone : la closure prend `&Observer` et retourne un Result.
    Sync(Arc<dyn Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static>),

    /// Exécution asynchrone : la closure prend un Observer (par valeur) et retourne une Future.
    /// La future est boxée et devra être conduite par subscribe() (ici on la drive dans un thread).
    Async(Arc<
        dyn Fn(Observer<TValue, TError>) -> Pin<Box<dyn Future<Output = Result<(), TError>> + Send>>
            + Send
            + Sync
            + 'static,
    >),
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