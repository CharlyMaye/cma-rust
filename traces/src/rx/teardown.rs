//https://refactoring.guru/design-patterns/observer
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::rx::observer::Observer;

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
    pub fn from_async<F, Fut>(f: F) -> Self
    where
        F: Fn(Observer<TValue, TError>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TError>> + Send + 'static,
    {
        TeardownLogic::Async(Arc::new(move |obs| Box::pin(f(obs))))
    }

    pub fn from_sync<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        TeardownLogic::Sync(Arc::new(f))
    }
}