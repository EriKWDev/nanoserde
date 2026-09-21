#[cfg(feature = "std")]
mod switch {
    use core::cell::Cell;

    std::thread_local! {
        static INCLUDED: Cell<bool> = const { Cell::new(false) };
    }

    pub fn included() -> bool {
        INCLUDED.with(|included| included.get())
    }

    struct Restore(bool);

    impl Drop for Restore {
        fn drop(&mut self) {
            INCLUDED.with(|included| included.set(self.0));
        }
    }

    pub fn while_included<R>(included_now: bool, body: impl FnOnce() -> R) -> R {
        let _restore = Restore(INCLUDED.with(|included| included.replace(included_now)));
        body()
    }
}

#[cfg(not(feature = "std"))]
mod switch {
    pub fn included() -> bool {
        false
    }

    pub fn while_included<R>(_included_now: bool, body: impl FnOnce() -> R) -> R {
        body()
    }
}

pub use switch::{included, while_included};
