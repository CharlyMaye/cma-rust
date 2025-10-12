use std::future::Future;
use std::task::{RawWaker, RawWakerVTable, Waker, Context, Poll};
use std::sync::{Arc, Mutex, Condvar};


// small standalone block_on (no external crate)
// suitable for driving a single future to completion (example/test usage)
pub(crate) fn block_on<F: Future>(f: F) -> F::Output {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let data = Arc::into_raw(pair.clone()) as *const ();

    unsafe fn raw_clone(data: *const ()) -> RawWaker {
        let arc = Arc::from_raw(data as *const (Mutex<bool>, Condvar));
        let arc2 = arc.clone();
        let _ = Arc::into_raw(arc);
        RawWaker::new(Arc::into_raw(arc2) as *const (), &VTABLE)
    }

    unsafe fn raw_wake(data: *const ()) {
        let arc = Arc::from_raw(data as *const (Mutex<bool>, Condvar));
        let (lock, cvar) = &*arc;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }

    unsafe fn raw_wake_by_ref(data: *const ()) {
        let arc = Arc::from_raw(data as *const (Mutex<bool>, Condvar));
        {
            let (lock, cvar) = &*arc;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        }
        let _ = Arc::into_raw(arc);
    }

    unsafe fn raw_drop(data: *const ()) {
        let _ = Arc::from_raw(data as *const (Mutex<bool>, Condvar));
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(
        raw_clone,
        raw_wake,
        raw_wake_by_ref,
        raw_drop,
    );

    let raw = RawWaker::new(data, &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);

    let mut fut = Box::pin(f);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => {
                unsafe { let _ = Arc::from_raw(data as *const (Mutex<bool>, Condvar)); }
                return v;
            }
            Poll::Pending => {
                let (lock, cvar) = &*pair;
                let mut started = lock.lock().unwrap();
                while !*started {
                    started = cvar.wait(started).unwrap();
                }
                *started = false;
            }
        }
    }
}
