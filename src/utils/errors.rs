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