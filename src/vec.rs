//! Slice Vector
//!
//! The struct is extracted from crate `tinyvec`
use core::{
    iter::FusedIterator,
    mem,
    ops::{Deref, DerefMut},
};

/// A slice-backed vector-like data structure.
pub struct SliceVec<'s, T> {
    data: &'s mut [T],
    len: usize,
}

impl<'s, T> Default for SliceVec<'s, T> {
    #[inline(always)]
    #[must_use]
    fn default() -> Self {
        Self {
            data: &mut [],
            len: 0,
        }
    }
}

impl<'s, T> Deref for SliceVec<'s, T> {
    type Target = [T];
    #[inline(always)]
    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.data[..self.len]
    }
}

impl<'s, T> DerefMut for SliceVec<'s, T> {
    #[inline(always)]
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..self.len]
    }
}

impl<'s, T> From<&'s mut [T]> for SliceVec<'s, T> {
    /// Uses the full slice as the initial length.
    fn from(data: &'s mut [T]) -> Self {
        let len = data.len();
        Self { data, len }
    }
}

impl<'s, T, A> From<&'s mut A> for SliceVec<'s, T>
where
    A: AsMut<[T]>,
{
    /// Calls `AsRef::as_mut` then uses the full slice as the initial length.
    fn from(a: &'s mut A) -> Self {
        let data = a.as_mut();
        let len = data.len();
        Self { data, len }
    }
}

impl<'s, T> SliceVec<'s, T> {
    /// The capacity of the `SliceVec`.
    #[inline(always)]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Extend from slice
    #[inline]
    pub fn extend_from_slice(&mut self, sli: &[T])
    where
        T: Clone,
    {
        if sli.is_empty() {
            return;
        }

        let new_len = self.len + sli.len();
        if new_len > self.capacity() {
            panic!(
                "SliceVec::extend_from_slice> total length {} exceeds capacity {}",
                new_len,
                self.capacity()
            )
        }

        let target = &mut self.data[self.len..new_len];
        target.clone_from_slice(sli);
        self.set_len(new_len);
    }

    /// Set the length
    #[inline(always)]
    pub fn set_len(&mut self, new_len: usize) {
        if new_len > self.capacity() {
            // Note(Lokathor): Technically we don't have to panic here, and we could
            // just let some other call later on trigger a panic on accident when the
            // length is wrong. However, it's a lot easier to catch bugs when things
            // are more "fail-fast".
            panic!(
                "SliceVec::set_len> new length {} exceeds capacity {}",
                new_len,
                self.capacity()
            )
        } else {
            self.len = new_len;
        }
    }
}

/// Draining iterator for [`SliceVec`]
///
/// See [`SliceVec::drain`](SliceVec::drain)
pub struct SliceVecDrain<'p, 's, T: Default> {
    parent: &'p mut SliceVec<'s, T>,
    target_start: usize,
    target_index: usize,
    target_end: usize,
}
impl<'p, 's, T: Default> Iterator for SliceVecDrain<'p, 's, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.target_index != self.target_end {
            let out = mem::take(&mut self.parent[self.target_index]);
            self.target_index += 1;
            Some(out)
        } else {
            None
        }
    }
}
impl<'p, 's, T: Default> FusedIterator for SliceVecDrain<'p, 's, T> {}
impl<'p, 's, T: Default> Drop for SliceVecDrain<'p, 's, T> {
    #[inline]
    fn drop(&mut self) {
        // Changed because it was moving `self`, it's also more clear and the std
        // does the same
        self.for_each(drop);
        // Implementation very similar to [`SliceVec::remove`](SliceVec::remove)
        let count = self.target_end - self.target_start;
        let targets: &mut [T] = &mut self.parent.deref_mut()[self.target_start..];
        targets.rotate_left(count);
        self.parent.len -= count;
    }
}
