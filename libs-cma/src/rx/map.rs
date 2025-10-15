use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;

use crate::rx::observable::Observable;
use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;


impl<TValue, TError> Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    /// Opérateur map - transforme chaque valeur émise
    /// Ne s'exécute que lors du subscribe()
    pub fn map<U, F>(self, mapper: F) -> Observable<U, TError>
    where
        U: 'static + Send,
        F: Fn(TValue) -> U + Send + Sync + 'static,
    {
        let source_teardown = self.teardown;
        let mapper = Arc::new(mapper);

        // On crée un nouvel Observable qui "enregistre" la transformation
        match source_teardown {
            TeardownLogic::Sync(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_sync(move |observer: &Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);
                        let observer_next = Arc::clone(&observer.next);
                        let observer_error = Arc::clone(&observer.error);
                        let observer_complete = Arc::clone(&observer.complete);
                        let observer_active = Arc::clone(&observer.active);

                        // Observer intermédiaire qui applique map
                        let inner_observer = Observer {
                            next: Arc::new(move |value: TValue| {
                                let mapped = mapper(value);
                                observer_next(mapped);
                            }),
                            error: observer_error,
                            complete: observer_complete,
                            active: observer_active,
                        };

                        // Subscribe à la source
                        source_fn(&inner_observer)
                    }),
                }
            }
            TeardownLogic::Async(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_async(move |observer: Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);
                        let observer_next = Arc::clone(&observer.next);
                        let observer_error = Arc::clone(&observer.error);
                        let observer_complete = Arc::clone(&observer.complete);
                        let observer_active = Arc::clone(&observer.active);

                        // Observer intermédiaire qui applique map
                        let inner_observer = Observer {
                            next: Arc::new(move |value: TValue| {
                                let mapped = mapper(value);
                                observer_next(mapped);
                            }),
                            error: observer_error,
                            complete: observer_complete,
                            active: observer_active,
                        };

                        // Subscribe à la source (async)
                        source_fn(inner_observer)
                    }),
                }
            }
        }
    }


}

