//! Yield the bits of a source flags value in a set of contained flags values.

use core::iter::FusedIterator;

use super::Flags;

/// An iterator over flags values.
///
/// This iterator only yields flags values for contained, defined, named flags. Any remaining bits
/// won't be yielded, but can be found with the [`#iter_name_ty::remaining`] method.
pub struct IterNames<B: 'static> {
    flags: &'static [(&'static str, B)],
    index: usize,
    source: B,
    remaining: B,
}

impl<B: Flags> IterNames<B> {
    #[inline]
    pub(crate) fn new(flags: &B) -> Self {
        Self {
            flags: B::KNOWN_FLAGS,
            index: 0,
            source: B::from_bits_retain(flags.bits()),
            remaining: B::from_bits_retain(flags.bits()),
        }
    }
}

impl<B: 'static> IterNames<B> {
    /// Get a flags value of any remaining bits that haven't been yielded yet.
    ///
    /// Once the iterator has finished, this method can be used to
    /// check whether or not there are any bits that didn't correspond
    /// to a contained, defined, named flag remaining.
    #[inline]
    pub const fn remaining(&self) -> &B {
        &self.remaining
    }

    #[doc(hidden)]
    #[inline]
    pub const fn __private_const_new(
        flags: &'static [(&'static str, B)],
        source: B,
        remaining: B,
    ) -> Self {
        IterNames {
            flags,
            index: 0,
            remaining,
            source,
        }
    }
}

impl<B: Flags> Iterator for IterNames<B> {
    type Item = (&'static str, B);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((name, flag)) = self.flags.get(self.index) {
            // Short-circuit if our state is empty
            if self.remaining.is_empty() {
                return None;
            }

            self.index += 1;

            // If the flag is set in the original source _and_ it has bits that haven't
            // been covered by a previous flag yet then yield it. These conditions cover
            // two cases for multi-bit flags:
            //
            // 1. When flags partially overlap, such as `0b00000001` and `0b00000101`, we'll
            // yield both flags.
            // 2. When flags fully overlap, such as in convenience flags that are a shorthand for others,
            // we won't yield both flags.
            if self.source.contains(*flag) && self.remaining.intersects(*flag) {
                self.remaining.unset(*flag);

                return Some((name, B::from_bits_retain(flag.bits())));
            }
        }

        None
    }
}

impl<B: Flags> FusedIterator for IterNames<B> {}

/// An iterator over flags values.
///
/// This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
/// as a final flags value.
pub struct Iter<B: 'static> {
    inner: IterNames<B>,
    done: bool,
}

impl<B: Flags> Iter<B> {
    #[inline]
    pub fn new(flags: &B) -> Self {
        Self {
            inner: IterNames::new(flags),
            done: false,
        }
    }
}

impl<B: 'static> Iter<B> {
    // Used by the `bitflags` macro
    #[doc(hidden)]
    #[inline]
    pub const fn __private_const_new(
        flags: &'static [(&'static str, B)],
        source: B,
        remaining: B,
    ) -> Self {
        Iter {
            inner: IterNames::__private_const_new(flags, source, remaining),
            done: false,
        }
    }
}

impl<B: Flags> Iterator for Iter<B> {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, flag)) => Some(flag),
            None if !self.done => {
                self.done = true;

                // After iterating through valid names, if there are any bits left over
                // then return one final value that includes them. This makes `into_iter`
                // and `from_iter` roundtrip
                if !self.inner.remaining().is_empty() {
                    Some(self.inner.remaining)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<B: Flags> FusedIterator for Iter<B> {}
