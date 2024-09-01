use super::*;
use crate::{col::ColRef, unzipped, zipped};

/// Diagonal matrix view.
#[derive(Debug)]
pub struct DiagRef<'a, E: Entity> {
    pub(crate) inner: ColRef<'a, E>,
}

impl<'a, E: Entity> DiagRef<'a, E> {
    /// Returns the diagonal as a column vector view.
    #[inline(always)]
    pub fn column_vector(self) -> ColRef<'a, E> {
        self.inner
    }

    /// Returns a view over the matrix.
    #[inline]
    pub fn as_ref(&self) -> DiagRef<'_, E> {
        *self
    }
}

impl<'a, E: Conjugate> DiagRef<'a, E> 
where
    E::Canonical: ComplexField,
{
    /// Returns the inverse of this diagonal matrix.
    pub fn inverse(self) -> Diag<E::Canonical> {
        Diag { inner: zipped!(self.inner).map(|unzipped!(value)| value.read().canonicalize().faer_inv()) }
    }
}

impl<E: Entity> Clone for DiagRef<'_, E> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<E: Entity> Copy for DiagRef<'_, E> {}

impl<'short, E: Entity> Reborrow<'short> for DiagRef<'_, E> {
    type Target = DiagRef<'short, E>;

    #[inline]
    fn rb(&'short self) -> Self::Target {
        *self
    }
}

impl<'short, E: Entity> ReborrowMut<'short> for DiagRef<'_, E> {
    type Target = DiagRef<'short, E>;

    #[inline]
    fn rb_mut(&'short mut self) -> Self::Target {
        *self
    }
}

impl<E: Entity> IntoConst for DiagRef<'_, E> {
    type Target = Self;

    #[inline]
    fn into_const(self) -> Self::Target {
        self
    }
}
