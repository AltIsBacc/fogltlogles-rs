use std::ops::Deref;

/// A zero-cost wrapper that implements Send and Sync for any type.
/// 
/// # Safety
/// You must ensure that mutations to the underlying type are synchronized
/// or that the type is read-only after initialization.
pub struct UnsafeSendSync<T>(pub T);

unsafe impl<T> Sync for UnsafeSendSync<T> {}
unsafe impl<T> Send for UnsafeSendSync<T> {}

impl<T> Deref for UnsafeSendSync<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

