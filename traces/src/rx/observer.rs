//https://refactoring.guru/design-patterns/observer

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
