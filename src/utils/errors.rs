pub(crate) trait Check {
    fn check(&self) -> bool;
}

pub(crate) trait OrFail<E> where Self: Check {
    #[inline(always)]
    fn or_fail(&self, error: E) -> Result<(), E> {
        if self.check() {
            Ok(())
        } else {
            Err(error)
        }
    }
}
impl<T, E> OrFail<E> for T where T: Check {}

pub(crate) trait OrFailWith<F, E> where Self: Check, F: FnOnce() -> E {
    #[inline(always)]
    fn or_fail_with(&self, handler: F) -> Result<(), E> {
        if self.check() {
            Ok(())
        } else {
            Err(handler())
        }
    }
}
impl<T, F, E> OrFailWith<F, E> for T where T: Check, F: FnOnce() -> E {}

impl Check for bool {
    #[inline(always)]
    fn check(&self) -> bool {
        *self
    }
}

pub(crate) trait ToResult<T, E> {
    // Add an erro message to an Option type: convert an Option<T> type into a
    // Result<T, E>
    fn into_result<F>(self, f: F) -> Result<T, E> where F: FnOnce() -> E;
}

impl<T, E> ToResult<T, E> for Option<T> {
    fn into_result<F>(self, f: F) -> Result<T, E> where F: FnOnce() -> E{
        self.map_or_else(|| Err(f()), |v| Ok(v))
    }
}

pub(crate) trait WrapError<T, E> {
    fn wrap_error(self) -> Result<T, E>;
}

impl<T, Es, Et> WrapError<T, Et> for Result<T, Es> where Et: From<Es> {
    fn wrap_error(self) -> Result<T, Et> {
        self.map_err(|e| e.into())
    }
}