//https://refactoring.guru/design-patterns/observer
use std::sync::Arc;

pub struct Observer<TValue, TError> {
    pub next: Arc<dyn Fn(TValue) + Send + Sync + 'static>,
    pub error: Arc<dyn Fn(TError) + Send + Sync + 'static>,
    pub complete: Arc<dyn Fn() + Send + Sync + 'static>,
}
impl<TValue, TError> Clone for Observer<TValue, TError> {
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            error: self.error.clone(),
            complete: self.complete.clone(),
        }
    }
}
