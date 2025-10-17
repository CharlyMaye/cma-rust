//! Observer implementation for the reactive programming patterns.
//!
//! Based on the Observer design pattern: https://refactoring.guru/design-patterns/observer

use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool};

/// Observer represents the recipient of notifications from an Observable.
///
/// The Observer contains three callback functions:
/// - `next`: Called when the Observable emits a new value
/// - `error`: Called when an error occurs in the Observable
/// - `complete`: Called when the Observable completes successfully
///
/// All callbacks are "wrapped" - they check the `active` flag before
/// calling the user-provided closure to ensure callbacks don't execute
/// after unsubscription.
///
/// # Generic Parameters
///
/// * `TValue` - The type of values emitted by the Observable
/// * `TError` - The type of errors that can occur
pub struct Observer<TValue, TError> {
    /// Callback for handling next values from the Observable
    pub next: Arc<dyn Fn(TValue) + Send + Sync + 'static>,
    /// Callback for handling errors from the Observable
    pub error: Arc<dyn Fn(TError) + Send + Sync + 'static>,
    /// Callback for handling completion of the Observable
    pub complete: Arc<dyn Fn() + Send + Sync + 'static>,

    /// Cancellation token shared between Unsubscribable and Observer.
    /// When set to false, callbacks will not execute.
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
    /// Helper method to check if this observer is still active.
    ///
    /// This provides a safe interface to access the cancellation token
    /// without directly exposing atomic operations.
    ///
    /// # Returns
    ///
    /// `true` if the observer is still active and should process events,
    /// `false` if it has been unsubscribed
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}
