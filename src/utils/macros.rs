#[macro_export]
macro_rules! try_continue {
    ($opt:expr) => {
        if let Some(v) = $opt {
            v
        } else {
            continue;
        }
    };
}

#[macro_export]
macro_rules! try_continue_res {
    ($opt:expr) => {
        if let Ok(v) = $opt {
            v
        } else {
            continue;
        }
    };
}
