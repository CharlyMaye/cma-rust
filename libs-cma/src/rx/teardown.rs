//! Teardown logic for Observable subscriptions.
//! 
//! This module defines the teardown behavior for Observable subscriptions,
//! supporting both synchronous and asynchronous execution patterns.
//! 
//! Based on the Observer design pattern: https://refactoring.guru/design-patterns/observer

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::rx::observer::Observer;

/// A boxed Future representing an asynchronous teardown operation
type AsyncTeardownFuture<TError> = Pin<Box<dyn Future<Output = Result<(), TError>> + Send>>;
/// Result type for synchronous teardown operations
type SyncTeardownResult<TError> = Result<(), TError>;

/// Type alias for a synchronous teardown function.
/// 
/// Takes a reference to an Observer and returns a Result immediately.
type SyncTeardownFn<TValue, TError> =
    Arc<dyn Fn(&Observer<TValue, TError>) -> SyncTeardownResult<TError> + Send + Sync + 'static>;

/// Type alias for an asynchronous teardown function.
/// 
/// Takes ownership of an Observer and returns a Future that must be polled to completion.
type AsyncTeardownFn<TValue, TError> =
    Arc<dyn Fn(Observer<TValue, TError>) -> AsyncTeardownFuture<TError> + Send + Sync + 'static>;

/// Defines the teardown logic for Observable subscriptions.
/// 
/// TeardownLogic encapsulates how an Observable should execute when subscribed to.
/// It supports two execution modes:
/// 
/// - **Sync**: Executes immediately and synchronously when subscribed
/// - **Async**: Returns a Future that must be driven to completion on a background thread
/// 
/// # Generic Parameters
/// 
/// * `TValue` - The type of values emitted by the Observable
/// * `TError` - The type of errors that can occur during execution
pub enum TeardownLogic<TValue, TError> {
    /// Synchronous execution: the closure takes `&Observer` and returns a Result.
    /// Execution completes immediately when the function returns.
    Sync(SyncTeardownFn<TValue, TError>),

    /// Asynchronous execution: the closure takes an Observer (by value) and returns a Future.
    /// The Future is boxed and must be driven by subscribe() (here we drive it in a thread).
    Async(AsyncTeardownFn<TValue, TError>),
}

impl<TValue, TError> std::fmt::Debug for TeardownLogic<TValue, TError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(_) => write!(f, "TeardownLogic::Sync"),
            Self::Async(_) => write!(f, "TeardownLogic::Async"),
        }
    }
}

impl<TValue, TError> TeardownLogic<TValue, TError> {
    /// Creates a new synchronous teardown logic from a closure.
    /// 
    /// The provided closure will be executed immediately when the Observable is subscribed to.
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes an Observer reference and returns a Result
    /// 
    /// # Returns
    /// 
    /// A TeardownLogic::Sync variant containing the wrapped closure
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cma::rx::teardown::TeardownLogic;
    /// 
    /// let teardown = TeardownLogic::from_sync(|observer| {
    ///     (observer.next)(42);
    ///     (observer.complete)();
    ///     Ok(())
    /// });
    /// ```
    pub fn from_sync<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        TeardownLogic::Sync(Arc::new(f))
    }

    /// Creates a new asynchronous teardown logic from a closure that returns a Future.
    /// 
    /// The provided closure will be executed on a background thread when the Observable 
    /// is subscribed to. The returned Future must be polled to completion.
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes an Observer by value and returns a Future
    /// 
    /// # Returns
    /// 
    /// A TeardownLogic::Async variant containing the wrapped closure
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cma::rx::teardown::TeardownLogic;
    /// 
    /// let teardown = TeardownLogic::from_async(|observer| async move {
    ///     // Simulate async work
    ///     tokio::time::sleep(Duration::from_millis(100)).await;
    ///     (observer.next)(42);
    ///     (observer.complete)();
    ///     Ok(())
    /// });
    /// ```
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
