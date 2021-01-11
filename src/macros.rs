use std::sync::atomic::{AtomicUsize, Ordering};

static PADDING: AtomicUsize = AtomicUsize::new(0);

pub fn get_padding() -> usize {
    PADDING.load(Ordering::SeqCst)
}

pub fn set_padding(value: usize) {
    PADDING.store(value, Ordering::SeqCst)
}

pub fn sub_padding(amount: usize) {
    let padding = get_padding();
    if padding >= amount {
        set_padding(padding - amount);
    }
}

pub fn padding() -> String {
    format!("{0:<1$}", " ", get_padding())
}

#[macro_export]
macro_rules! debug_log {
    ($msg:expr) => {
        if $crate::LOGGING_ENABLED {
            println!("{}{}", $crate::macros::padding(), $msg);
        }
    };
    ($($x:tt)*)=> {
        if $crate::LOGGING_ENABLED {
            println!("{}{}", $crate::macros::padding(), format!($($x)*));
        }
    };
}

#[macro_export]
macro_rules! debug_log_start {
    ($msg:expr) => {
        if $crate::LOGGING_ENABLED {
            debug_log!($msg);
            $crate::macros::set_padding($crate::macros::get_padding() + 2);
        }
    };
    ($($x:tt)*)=> {
        if $crate::LOGGING_ENABLED {
            debug_log!($($x)*);
            $crate::macros::set_padding($crate::macros::get_padding() + 2);
        }
    };
}

#[macro_export]
macro_rules! debug_log_end {
    () => {
        if $crate::LOGGING_ENABLED {
            $crate::macros::sub_padding(2);
        }
    };
    ($msg:expr) => {
        if $crate::LOGGING_ENABLED {
            $crate::macros::sub_padding(2);
            debug_log!($msg);
        }
    };
    ($($x:tt)*)=> {
        if $crate::LOGGING_ENABLED {
            $crate::macros::sub_padding(2);
            debug_log!($($x)*);
        }
    };
}
