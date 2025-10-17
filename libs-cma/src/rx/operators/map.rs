use std::sync::Arc;

use crate::rx::observable::Observable;
use crate::rx::observer::Observer;
use crate::rx::teardown::TeardownLogic;

// TODO - refactoriser la partie sync et async sur une prochaine PR
#[allow(dead_code)]
impl<TValue, TError> Observable<TValue, TError>
where
    TValue: 'static + Send,
    TError: 'static + Send,
{
    /// Transforme chaque valeur émise par l'Observable source.
    ///
    /// L'opérateur `map` applique une fonction de transformation à chaque valeur
    /// émise par l'Observable source et émet le résultat.
    ///
    /// # Exécution lazy
    /// Cet opérateur ne s'exécute que lors de l'appel à `subscribe()`.
    /// Il crée un nouvel Observable sans déclencher la source.
    ///
    /// # Exemples
    /// ```
    /// use cma::rx::Observable;
    ///
    /// let observable = Observable::new(|observer| {
    ///     (observer.next)(1);
    ///     (observer.next)(2);
    ///     (observer.next)(3);
    ///     (observer.complete)();
    ///     Ok(())
    /// })
    /// .map(|x| x * 2)
    /// .map(|x| x + 1);
    ///
    /// // Rien ne s'exécute avant subscribe()
    /// observable.subscribe(
    ///     |x| println!("{}", x), // Affichera: 3, 5, 7
    ///     |e| eprintln!("Erreur: {:?}", e),
    ///     || println!("Complété"),
    /// );
    /// ```
    ///
    /// # Paramètres
    /// - `mapper`: Fonction de transformation `Fn(TValue) -> U`
    ///
    /// # Retour
    /// Un nouvel `Observable<U, TError>` qui émet les valeurs transformées
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

                        // Création de l'Observer intermédiaire qui applique la transformation
                        let inner_observer = create_mapping_observer(observer, mapper);

                        // Subscribe à la source
                        source_fn(&inner_observer)
                    }),
                }
            }
            TeardownLogic::Async(source_fn) => {
                Observable {
                    teardown: TeardownLogic::from_async(move |observer: Observer<U, TError>| {
                        let mapper = Arc::clone(&mapper);

                        // Création de l'Observer intermédiaire qui applique la transformation
                        let inner_observer = create_mapping_observer_owned(observer, mapper);

                        // Subscribe à la source (async)
                        source_fn(inner_observer)
                    }),
                }
            }
        }
    }
}

/// Fonction helper pour créer un Observer qui applique une transformation
/// Version pour le cas Sync (prend une référence)
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

/// Fonction helper pour créer un Observer qui applique une transformation
/// Version pour le cas Async (prend ownership)
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
    use std::sync::{Arc, Mutex}; // Importer le trait

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
