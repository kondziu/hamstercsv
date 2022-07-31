pub(crate) fn check_or<E>(condition: bool, error: E) -> Result<(), E> {
    if condition {
        Ok(())
    } else {
        Err(error)
    }
}

pub(crate) fn check_or_else<F, E>(condition: bool, error: F) -> Result<(), E> where F: FnOnce() -> E {
    if condition {
        Ok(())
    } else {
        Err(error())
    }
}