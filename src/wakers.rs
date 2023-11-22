use std::{
    sync::LazyLock,
    task::Waker,
    thread::{self, Thread},
};

/// Creates a dummy waker that does nothing.
pub(crate) fn empty_waker() -> Waker {
    static WAKER: LazyLock<Waker> = LazyLock::new(|| waker_fn::waker_fn(move || {}));

    WAKER.clone()
}

/// Creates a waker that unparks the current thread.
pub(crate) fn current_thread_waker() -> Waker {
    thread_waker(thread::current())
}

/// Creates a waker that unparks a thread.
pub(crate) fn thread_waker(thread: Thread) -> Waker {
    waker_fn::waker_fn(move || thread.unpark())
}
