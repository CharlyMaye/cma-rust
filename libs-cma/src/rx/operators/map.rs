use std::sync::Arc;

use crate::rx::observable::Observable;
use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;

// TODO: Refactor the sync and async parts in a future PR
#[allow(dead_code)]
impl<TValue, TError> Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    /// Transforms each value emitted by the source Observable.
    ///
    /// The `map` operator applies a transformation function to each value
    /// emitted by the source Observable and emits the result.
    ///
    /// # Lazy Execution
    /// This operator only executes when `subscribe()` is called.
    /// It creates a new Observable without triggering the source.
    ///
    /// # Examples
    /// ```no_run
    /// use traces::rx::observable::{Observable, Subscribable};
    ///
    /// let mut observable: Observable<i32, ()> = Observable::new(|observer| {
    ///     (observer.next)(1);
    ///     (observer.next)(2);
    ///     (observer.next)(3);
    ///     (observer.complete)();
    ///     Ok(())
    /// })
    /// .map(|x| x * 2)
    /// .map(|x| x + 1);
    ///
    /// // Nothing executes before subscribe()
    /// observable.subscribe(
    ///     |x| println!("{}", x), // Will print: 3, 5, 7
    ///     |e| eprintln!("Error: {:?}", e),
    ///     || println!("Completed"),
    /// );
    /// ```
    ///
    /// # Parameters
    /// - `mapper`: Transformation function `Fn(TValue) -> U`
    ///
    /// # Returns
    /// A new `Observable<U, TError>` that emits the transformed values
    pub fn map<U, F>(self, mapper: F) -> Observable<U, TError>
    where
        U: 'static + Send,
        F: Fn(TValue) -> U + Send + Sync + 'static,
    {
        let source_teardown = self.teardown;
        let mapper = Arc::new(mapper);

        // Create a new Observable that "registers" the transformation
        match source_teardown {
            TeardownLogic::Sync(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_sync(move |observer: &Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);

                        // Create intermediate Observer that applies the transformation
                        let inner_observer = create_mapping_observer(observer, mapper);

                        // Subscribe to the source
                        source_fn(&inner_observer)
                    }),
                }
            }
            TeardownLogic::Async(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_async(move |observer: Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);

                        // Create intermediate Observer that applies the transformation
                        let inner_observer = create_mapping_observer_owned(observer, mapper);

                        // Subscribe to the source (async)
                        source_fn(inner_observer)
                    }),
                }
            }
        }
    }
}

/// Helper function to create an Observer that applies a transformation.
/// Version for the Sync case (takes a reference).
fn create_mapping_observer<TValue, U, TError, F>(
    observer: &Observer<U, TError>,
    mapper: Arc<F>,
) -> Observer<TValue, TError>
where
    TValue: 'static,
    U: 'static,
    TError: 'static,
    F: Fn(TValue) -> U + Send + Sync + 'static,
{
    let observer_next = Arc::clone(&observer.next);
    let observer_error = Arc::clone(&observer.error);
    let observer_complete = Arc::clone(&observer.complete);
    let observer_active = Arc::clone(&observer.active);

    Observer {
        next: Arc::new(move |value: TValue| {
            let mapped = mapper(value);
            observer_next(mapped);
        }),
        error: observer_error,
        complete: observer_complete,
        active: observer_active,
    }
}

/// Helper function to create an Observer that applies a transformation.
/// Version for the Async case (takes ownership).
fn create_mapping_observer_owned<TValue, U, TError, F>(
    observer: Observer<U, TError>,
    mapper: Arc<F>,
) -> Observer<TValue, TError>
where
    TValue: 'static,
    U: 'static,
    TError: 'static,
    F: Fn(TValue) -> U + Send + Sync + 'static,
{
    let observer_next = Arc::clone(&observer.next);
    let observer_error = Arc::clone(&observer.error);
    let observer_complete = Arc::clone(&observer.complete);
    let observer_active = Arc::clone(&observer.active);

    Observer {
        next: Arc::new(move |value: TValue| {
            let mapped = mapper(value);
            observer_next(mapped);
        }),
        error: observer_error,
        complete: observer_complete,
        active: observer_active,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rx::observable::Subscribable;
    use std::sync::{Arc, Mutex}; // Import the trait

    #[test]
    fn test_map_simple() {
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);

        let mut observable: Observable<i32, ()> = Observable::new(|observer| {
            (observer.next)(1);
            (observer.next)(2);
            (observer.next)(3);
            (observer.complete)();
            Ok(())
        })
        .map(|x| x * 2);

        let mut sub = observable.subscribe(
            move |x| results_clone.lock().unwrap().push(x),
            |_| {},
            || {},
        );

        sub.join().unwrap();
        assert_eq!(*results.lock().unwrap(), vec![2, 4, 6]);
    }

    #[test]
    fn test_map_chained() {
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);

        let mut observable: Observable<String, ()> = Observable::new(|observer| {
            (observer.next)(1);
            (observer.next)(2);
            (observer.next)(3);
            (observer.complete)();
            Ok(())
        })
        .map(|x| x * 2)
        .map(|x| x + 1)
        .map(|x| format!("value: {}", x));

        let mut sub = observable.subscribe(
            move |x| results_clone.lock().unwrap().push(x),
            |_| {},
            || {},
        );

        sub.join().unwrap();
        assert_eq!(
            *results.lock().unwrap(),
            vec!["value: 3", "value: 5", "value: 7"]
        );
    }

    #[test]
    fn test_map_type_conversion() {
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);

        let mut observable: Observable<f64, ()> = Observable::new(|observer| {
            (observer.next)(10);
            (observer.next)(20);
            (observer.complete)();
            Ok(())
        })
        .map(|x| x as f64)
        .map(|x| x / 3.0);

        let mut sub = observable.subscribe(
            move |x| results_clone.lock().unwrap().push(x),
            |_| {},
            || {},
        );

        sub.join().unwrap();
        let res = results.lock().unwrap();
        assert!((res[0] - 3.333333).abs() < 0.001);
        assert!((res[1] - 6.666666).abs() < 0.001);
    }
}
