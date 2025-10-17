//https://refactoring.guru/design-patterns/observer
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool};

pub struct Observer<TValue, TError> {
    // closures "wrappées" : elles vérifient le flag `active` avant d'appeler la closure utilisateur
    pub next: Arc<dyn Fn(TValue) + Send + Sync + 'static>,
    pub error: Arc<dyn Fn(TError) + Send + Sync + 'static>,
    pub complete: Arc<dyn Fn() + Send + Sync + 'static>,

    // token d'annulation partagé entre Unsubscribable et l'Observer
    pub active: Arc<AtomicBool>,
}

impl<TValue, TError> Clone for Observer<TValue, TError> {
    fn clone(&self) -> Self {
        Observer {
            next: Arc::clone(&self.next),
            error: Arc::clone(&self.error),
            complete: Arc::clone(&self.complete),
            active: Arc::clone(&self.active),
        }
    }
}
#[allow(dead_code)]
impl<TValue, TError> Observer<TValue, TError> {
    /// helper to check if this observer is still active (confinement de l'accès au token)
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}
