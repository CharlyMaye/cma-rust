//! Observable implementation for reactive programming patterns.
//!
//! This module provides the core Observable type that represents a stream of data
//! that can be observed. It supports both synchronous and asynchronous execution
//! patterns and provides subscription management.
//!
//! Based on the Observer design pattern: https://refactoring.guru/design-patterns/observer

use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;

use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;

// TODO: We're managing unsafe ourselves. It might be preferable to use the futures crate

/// Represents a subscription that can be unsubscribed from.
///
/// Unsubscribable indicates whether the operation is already finished (sync) or if it
/// is running in the background (async). We use a struct containing Option<JoinHandle>
/// to be able to take the handle (Option::take) without moving a field from a type that
/// implements Drop (avoids the "cannot move out of type ... which implements Drop" error).
#[allow(dead_code)]
pub struct Unsubscribable {
    /// Optional handle to the background thread (None for sync operations)
    handle: Option<std::thread::JoinHandle<()>>,
    /// Shared flag indicating if the subscription is still active
    active: Arc<AtomicBool>,
}
#[allow(dead_code)]
impl Unsubscribable {
    /// Creates a new Unsubscribable for synchronous operations (already completed).
    ///
    /// # Returns
    ///
    /// An Unsubscribable with no background handle and inactive flag
    pub fn ready() -> Self {
        Unsubscribable {
            handle: None,
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Creates a new Unsubscribable for asynchronous operations with a background thread.
    ///
    /// # Arguments
    ///
    /// * `handle` - The JoinHandle for the background thread
    /// * `active` - Shared atomic flag for cancellation
    ///
    /// # Returns
    ///
    /// An Unsubscribable managing the background operation
    pub fn background(handle: std::thread::JoinHandle<()>, active: Arc<AtomicBool>) -> Self {
        Unsubscribable {
            handle: Some(handle),
            active,
        }
    }

    /// Stop further callbacks immediately (non-blocking).
    ///
    /// The background thread may still run, but callbacks are ignored
    /// because `active` is set to false.
    pub fn unsubscribe(&mut self) {
        self.active.store(false, Ordering::SeqCst);
    }

    /// Stop callbacks and wait for background task to finish (blocking).
    ///
    /// This method will block until the background thread completes.
    ///
    /// # Returns
    ///
    /// Result of joining the background thread, or Ok(()) if no background thread
    pub fn unsubscribe_and_wait(&mut self) -> std::thread::Result<()> {
        self.active.store(false, Ordering::SeqCst);
        self.join()
    }

    /// Wait for background task to finish (if any).
    ///
    /// # Returns
    ///
    /// Result of joining the background thread, or Ok(()) if no background thread
    pub fn join(&mut self) -> std::thread::Result<()> {
        if let Some(h) = self.handle.take() {
            h.join()
        } else {
            Ok(())
        }
    }

    /// Detach: join in a spawned thread so this call is non-blocking.
    ///
    /// The background thread will be joined in a separate thread, allowing
    /// this method to return immediately.
    pub fn detach(&mut self) {
        if let Some(h) = self.handle.take() {
            std::thread::spawn(move || {
                let _ = h.join();
            });
        }
    }
}

impl Drop for Unsubscribable {
    fn drop(&mut self) {
        // Ensure callbacks are disabled on drop
        self.active.store(false, Ordering::SeqCst);
        if let Some(h) = self.handle.take() {
            // Don't block Drop: detach the join
            std::thread::spawn(move || {
                let _ = h.join();
            });
        }
    }
}

/// Observable represents a stream of data that can be observed.
///
/// An Observable is a lazy data structure that doesn't emit values until
/// it's subscribed to. It can emit zero or more values over time, and
/// may complete successfully or with an error.
///
/// # Generic Parameters
///
/// * `TValue` - The type of values emitted by the Observable
/// * `TError` - The type of errors that can occur
///
/// # Examples
///
/// ```
/// use cma::rx::Observable;
///
/// let observable = Observable::new(|observer| {
///     (observer.next)(42);
///     (observer.complete)();
///     Ok(())
/// });
///
/// observable.subscribe(
///     |value| println!("Received: {}", value),
///     |error| eprintln!("Error: {:?}", error),
///     || println!("Completed"),
/// );
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct Observable<TValue: 'static, TError: 'static> {
    /// The teardown logic that defines how this Observable executes
    // TODO: Should this be protected?
    pub teardown: TeardownLogic<TValue, TError>,
}
#[allow(dead_code)]
impl<TValue: 'static, TError: 'static> Observable<TValue, TError> {
    /// Creates a new Observable with synchronous execution.
    ///
    /// The provided function will be executed immediately when the Observable
    /// is subscribed to, and must complete synchronously.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes an Observer and returns a Result
    ///
    /// # Returns
    ///
    /// A new Observable that will execute the provided function on subscription
    ///
    /// # Examples
    ///
    /// ```
    /// use cma::rx::Observable;
    ///
    /// let observable = Observable::new(|observer| {
    ///     (observer.next)(1);
    ///     (observer.next)(2);
    ///     (observer.next)(3);
    ///     (observer.complete)();
    ///     Ok(())
    /// });
    /// ```
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Observer<TValue, TError>) -> Result<(), TError> + Send + Sync + 'static,
    {
        Observable {
            teardown: TeardownLogic::from_sync(f),
        }
    }

    /// Creates a new Observable with asynchronous execution.
    ///
    /// The provided function will be executed on a background thread when the Observable
    /// is subscribed to, and returns a Future that will be driven to completion.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes an Observer and returns a Future
    ///
    /// # Returns
    ///
    /// A new Observable that will execute the provided async function on subscription
    ///
    /// # Examples
    ///
    /// ```
    /// use cma::rx::Observable;
    ///
    /// let observable = Observable::with_async_teardown(|observer| async move {
    ///     // Simulate async work
    ///     tokio::time::sleep(Duration::from_millis(100)).await;
    ///     (observer.next)(42);
    ///     (observer.complete)();
    ///     Ok(())
    /// });
    /// ```
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

/// Trait for types that can be subscribed to.
///
/// This trait defines the subscription interface for Observable types,
/// allowing consumers to register callbacks for next values, errors, and completion.
#[allow(dead_code)]
pub trait Subscribable<TValue, TError> {
    /// Subscribe to this Observable with the provided callbacks.
    ///
    /// Takes three closures directly to avoid requiring Arc::new(...) on the caller side.
    ///
    /// # Arguments
    ///
    /// * `next` - Callback for handling emitted values
    /// * `error` - Callback for handling errors
    /// * `complete` - Callback for handling completion
    ///
    /// # Returns
    ///
    /// An Unsubscribable that can be used to cancel the subscription
    fn subscribe<N, E, C>(&mut self, next: N, error: E, complete: C) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static;
}
impl<TValue, TError> Subscribable<TValue, TError> for Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    fn subscribe<N, E, C>(&mut self, next: N, error: E, complete: C) -> Unsubscribable
    where
        N: Fn(TValue) + Send + Sync + 'static,
        E: Fn(TError) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static,
    {
        // Observer/wrappers (with `active`) are created in each branch (Sync/Async)
        // We don't instantiate `callbacks` here to avoid forgetting the `active` field

        match &self.teardown {
            TeardownLogic::Sync(arc_f) => {
                let f = arc_f.clone();
                // Create active flag true for sync as well (but will be used only if needed)
                let active = Arc::new(AtomicBool::new(true));
                let active_next = Arc::clone(&active);
                let active_err = Arc::clone(&active);
                let active_complete = Arc::clone(&active);

                let user_next = std::sync::Arc::new(next);
                let user_err = std::sync::Arc::new(error);
                let user_complete = std::sync::Arc::new(complete);

                let callbacks = Observer {
                    next: {
                        let u = Arc::clone(&user_next);
                        Arc::new(move |v| {
                            if active_next.load(Ordering::SeqCst) {
                                (u)(v)
                            }
                        })
                    },
                    error: {
                        let u = Arc::clone(&user_err);
                        Arc::new(move |e| {
                            if active_err.load(Ordering::SeqCst) {
                                (u)(e)
                            }
                        })
                    },
                    complete: {
                        let u = Arc::clone(&user_complete);
                        Arc::new(move || {
                            if active_complete.load(Ordering::SeqCst) {
                                (u)()
                            }
                        })
                    },
                    active: Arc::clone(&active),
                };

                match (f)(&callbacks) {
                    Ok(()) => Unsubscribable::ready(),
                    Err(e) => {
                        (callbacks.error)(e);
                        Unsubscribable::ready()
                    }
                }
            }
            TeardownLogic::Async(arc_f) => {
                let f = arc_f.clone();

                // Prepare active flag shared between observer wrappers and Unsubscribable
                let active = Arc::new(AtomicBool::new(true));
                let active_next = Arc::clone(&active);
                let active_err = Arc::clone(&active);
                let active_complete = Arc::clone(&active);

                let user_next = std::sync::Arc::new(next);
                let user_err = std::sync::Arc::new(error);
                let user_complete = std::sync::Arc::new(complete);

                // Observer whose callbacks check `active` before invoking user closures
                let cb_for_fut = Observer {
                    next: {
                        let u = Arc::clone(&user_next);
                        Arc::new(move |v| {
                            if active_next.load(Ordering::SeqCst) {
                                (u)(v)
                            }
                        })
                    },
                    error: {
                        let u = Arc::clone(&user_err);
                        Arc::new(move |e| {
                            if active_err.load(Ordering::SeqCst) {
                                (u)(e)
                            }
                        })
                    },
                    complete: {
                        let u = Arc::clone(&user_complete);
                        Arc::new(move || {
                            if active_complete.load(Ordering::SeqCst) {
                                (u)()
                            }
                        })
                    },
                    active: Arc::clone(&active),
                };

                // Second clone to be used only for error propagation by the driver
                let cb_for_err = cb_for_fut.clone();

                // Get the concrete future (boxed by TeardownLogic::from_async)
                let fut = (f)(cb_for_fut);

                // Spawn the background driver and return its handle
                let handle = spawn_driven_future(fut, cb_for_err);

                Unsubscribable::background(handle, active)
            }
        }
    }
}

// No-op RawWaker callbacks (they must have unsafe fn signatures)
#[allow(dead_code)]
unsafe fn noop_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &RAW_WAKER_VTABLE)
}
#[allow(dead_code)]
unsafe fn noop_wake(_data: *const ()) {}
#[allow(dead_code)]
unsafe fn noop_wake_by_ref(_data: *const ()) {}
#[allow(dead_code)]
unsafe fn noop_drop(_data: *const ()) {}

#[allow(dead_code)]
static RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(noop_clone, noop_wake, noop_wake_by_ref, noop_drop);

/// Helper that confines the single unsafe needed to create a Waker.
///
/// Creates a no-op waker that can be used to poll futures without
/// requiring a full async runtime.
#[allow(dead_code)]
fn create_noop_waker() -> Waker {
    let raw = RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw) }
}

/// Drive a boxed future to completion on a background thread, using a noop waker.
///
/// This function spawns a new thread that will poll the provided future until
/// it completes, using a simple polling loop with a no-op waker.
///
/// # Arguments
///
/// * `fut` - The boxed future to drive to completion
/// * `cb_for_err` - Observer for error propagation if the future fails
///
/// # Returns
///
/// A JoinHandle that the caller can wait on if desired
#[allow(dead_code)]
fn spawn_driven_future<TValue, TError>(
    mut fut: Pin<Box<dyn Future<Output = Result<(), TError>> + Send>>,
    cb_for_err: Observer<TValue, TError>,
) -> std::thread::JoinHandle<()>
where
    TError: Send + 'static,
    TValue: 'static,
    Observer<TValue, TError>: Send + 'static,
{
    thread::spawn(move || {
        let waker = create_noop_waker();
        let mut cx = Context::from_waker(&waker);

        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(Ok(())) => break,
                Poll::Ready(Err(e)) => {
                    (cb_for_err.error)(e);
                    break;
                }
                Poll::Pending => {
                    std::thread::yield_now();
                }
            }
        }
    })
}
