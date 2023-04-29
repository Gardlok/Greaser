use std::pin::Pin;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(crate) enum Parameter<T> {
    NotSet(T),
    AllSet(T),
}
pub(crate) type Param<T> = Parameter<T>;
impl<T> Parameter<T> {
    const fn is_set(&self) -> bool {
        matches!(*self, Param::AllSet(_))
    }
    pub(crate) fn is_set_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Param::NotSet(x) => f(x),
            Param::AllSet(x) => f(x),
        }
    }
    pub(crate) const fn as_ref(&self) -> Param<&T> {
        match *self {
            Param::AllSet(ref x) => Param::AllSet(x),
            Param::NotSet(ref x) => Param::NotSet(x),
        }
    }
    pub(crate) fn as_mut(&mut self) -> Param<&T> {
        match *self {
            Param::AllSet(ref mut x) => Param::AllSet(x),
            Param::NotSet(ref mut x) => Param::NotSet(x),
        }
    }
    pub(crate) fn as_pin_ref(self: Pin<&Self>) -> Param<Pin<&T>> {
        // pub(crate) const fn as_pin_ref(self: Pin<&Self>) -> CP<Pin<&T>> {
        match Pin::get_ref(self).as_ref() {
            Param::AllSet(x) => unsafe { Param::AllSet(Pin::new_unchecked(x)) },
            Param::NotSet(x) => unsafe { Param::NotSet(Pin::new_unchecked(x)) },
        }
    }
    pub(crate) fn as_pin_mut(self: Pin<&mut Self>) -> Param<Pin<&mut T>> {
        unsafe {
            match Pin::get_unchecked_mut(self) {
                Param::AllSet(x) => Param::AllSet(Pin::new_unchecked(x)),
                Param::NotSet(x) => Param::NotSet(Pin::new_unchecked(x)),
            }
        }
    }
    pub(crate) fn get(self) -> T {
        match self {
            Param::NotSet(x) => x,
            Param::AllSet(x) => x,
        }
    }
    pub(crate) fn set(&mut self, item: T) -> Result<(), ()> {
        match self {
            Param::NotSet(ref mut x) => *x = item,
            Param::AllSet(ref mut x) => *x = item,
        }; // TODO actual validation
        Ok(())
    }
    pub(crate) fn borrow(&self) -> &T {
        match *self {
            Param::NotSet(ref x) => &x,
            Param::AllSet(ref x) => &x,
        }
    }
    pub(crate) fn borrow_mut(&mut self) -> &T {
        match *self {
            Param::NotSet(ref mut x) => x,
            Param::AllSet(ref mut x) => x,
        }
    }
}

impl<T> std::fmt::Display for Parameter<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Param::AllSet(T) => write!(f, "Allset({})", T),
            Param::NotSet(T) => write!(f, "Notset({})", T),
        }
    }
}
