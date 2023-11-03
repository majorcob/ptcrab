use std::ops::Deref;

//--------------------------------------------------------------------------------------------------

/// Contains a <code>[Box]<\[T]></code> of length ≤ `N`.
///
/// Use [`into_vec`](Self::into_vec) to obtain a <code>[Vec]\<T></code>, which can later turn back
/// into a <code>[Box]<\[T]></code> using [`Vec::into_boxed_slice`].
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct MaxLen<const N: usize, T>(Box<[T]>);

/// [`MaxLen`] with `N` = [`i32::MAX`] (common max length in pxtone).
pub type MaxLenI32<T> = MaxLen<{ i32::MAX as usize }, T>;

impl<const N: usize, T> MaxLen<N, T> {
    /// Max item count. Slices with greater length than this will be rejected.
    pub const MAX: usize = N;

    /// Attempts to create a wrapper around a boxed slice of length ≤ `N`.
    pub fn new(items: Box<[T]>) -> Option<Self> {
        (items.len() <= Self::MAX).then_some(Self(items))
    }
}

impl<const N: usize, T> From<MaxLen<N, T>> for Box<[T]> {
    fn from(value: MaxLen<N, T>) -> Self {
        value.0
    }
}

impl<const N: usize, T> From<MaxLen<N, T>> for Vec<T> {
    fn from(value: MaxLen<N, T>) -> Self {
        value.0.into_vec()
    }
}

impl<const N: usize, T> Deref for MaxLen<N, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<const N: usize, T> Default for MaxLen<N, T> {
    fn default() -> Self {
        Self(Box::new([]))
    }
}
