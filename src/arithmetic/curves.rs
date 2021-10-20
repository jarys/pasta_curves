//! This module contains the `Curve`/`CurveAffine` abstractions that allow us to
//! write code that generalizes over a pair of groups.

use group::prime::{PrimeCurve, PrimeCurveAffine};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use super::{FieldExt, Group};

use core::ops::{Add, Mul, Sub};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
//#[cfg(feature = "std")]
//use std::io::{self, Read, Write};

/// This trait is a common interface for dealing with elements of an elliptic
/// curve group in a "projective" form, where that arithmetic is usually more
/// efficient.
///
/// Currently requires the `std` feature flag because of `hash_to_curve`, and
/// `CurveAffine::{read, write}`.
pub trait CurveExt:
    PrimeCurve<Affine = <Self as CurveExt>::AffineExt>
    + group::Group<Scalar = <Self as CurveExt>::ScalarExt>
    + Default
    + ConditionallySelectable
    + ConstantTimeEq
    + From<<Self as PrimeCurve>::Affine>
    + Group<Scalar = <Self as group::Group>::Scalar>
{
    /// The scalar field of this elliptic curve.
    type ScalarExt: FieldExt;
    /// The base field over which this elliptic curve is constructed.
    type Base: FieldExt;
    /// The affine version of the curve
    type AffineExt: CurveAffine<CurveExt = Self, ScalarExt = <Self as CurveExt>::ScalarExt>
        + Mul<Self::ScalarExt, Output = Self>
        + for<'r> Mul<Self::ScalarExt, Output = Self>;

    /// CURVE_ID used for hash-to-curve.
    const CURVE_ID: &'static str;

    /// Apply the curve endomorphism by multiplying the x-coordinate
    /// by an element of multiplicative order 3.
    fn endo(&self) -> Self;

    /// Return the Jacobian coordinates of this point.
    fn jacobian_coordinates(&self) -> (Self::Base, Self::Base, Self::Base);

    /// Requests a hasher that accepts messages and returns near-uniformly
    /// distributed elements in the group, given domain prefix `domain_prefix`.
    ///
    /// This method is suitable for use as a random oracle.
    ///
    /// # Example
    ///
    /// ```
    /// use pasta_curves::arithmetic::CurveExt;
    /// fn pedersen_commitment<C: CurveExt>(
    ///     x: C::ScalarExt,
    ///     r: C::ScalarExt,
    /// ) -> C::Affine {
    ///     let hasher = C::hash_to_curve("z.cash:example_pedersen_commitment");
    ///     let g = hasher(b"g");
    ///     let h = hasher(b"h");
    ///     (g * x + &(h * r)).to_affine()
    /// }
    /// ```
    #[cfg(feature = "alloc")]
    fn hash_to_curve<'a>(domain_prefix: &'a str) -> Box<dyn Fn(&[u8]) -> Self + 'a>;

    /// Unboxed version of hash_to_curve.
    ///
    /// # Example
    ///
    /// ```
    /// use pasta_curves::arithmetic::CurveExt;
    /// fn pedersen_commitment<C: CurveExt>(
    ///     x: C::ScalarExt,
    ///     r: C::ScalarExt,
    /// ) -> C::Affine {
    ///     let dst = "z.cash:example_pedersen_commitment";
    ///     let g = C::unboxed_hash_to_curve(dst, b"g");
    ///     let h = C::unboxed_hash_to_curve(dst, b"h");
    ///     (g * x + &(h * r)).to_affine()
    /// }
    /// ```
    fn unboxed_hash_to_curve(domain_prefix: &str, message: &[u8]) -> Self;

    /// Returns whether or not this element is on the curve; should
    /// always be true unless an "unchecked" API was used.
    fn is_on_curve(&self) -> Choice;

    /// Returns the curve constant a.
    fn a() -> Self::Base;

    /// Returns the curve constant b.
    fn b() -> Self::Base;

    /// Obtains a point given Jacobian coordinates $X : Y : Z$, failing
    /// if the coordinates are not on the curve.
    fn new_jacobian(x: Self::Base, y: Self::Base, z: Self::Base) -> CtOption<Self>;
}

/// This trait is the affine counterpart to `Curve` and is used for
/// serialization, storage in memory, and inspection of $x$ and $y$ coordinates.
pub trait CurveAffine:
    PrimeCurveAffine<
        Scalar = <Self as CurveAffine>::ScalarExt,
        Curve = <Self as CurveAffine>::CurveExt,
    > + Default
    + Add<Output = <Self as PrimeCurveAffine>::Curve>
    + Sub<Output = <Self as PrimeCurveAffine>::Curve>
    + ConditionallySelectable
    + ConstantTimeEq
    + From<<Self as PrimeCurveAffine>::Curve>
{
    /// The scalar field of this elliptic curve.
    type ScalarExt: FieldExt;
    /// The base field over which this elliptic curve is constructed.
    type Base: FieldExt;
    /// The projective form of the curve
    type CurveExt: CurveExt<AffineExt = Self, ScalarExt = <Self as CurveAffine>::ScalarExt>;

    /// Gets the coordinates of this point.
    ///
    /// Returns None if this is the identity.
    fn coordinates(&self) -> CtOption<Coordinates<Self>>;

    /// Obtains a point given $(x, y)$, failing if it is not on the
    /// curve.
    fn from_xy(x: Self::Base, y: Self::Base) -> CtOption<Self>;

    /// Returns whether or not this element is on the curve; should
    /// always be true unless an "unchecked" API was used.
    fn is_on_curve(&self) -> Choice;

    // TODO: move to TryFrom trait
    /// Reads a compressed element from the buffer and attempts to parse it
    /// using `from_bytes`.
    /*fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut compressed = Self::Repr::default();
        reader.read_exact(compressed.as_mut())?;
        Option::from(Self::from_bytes(&compressed))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid point encoding in proof"))
    }*/
    // TODO: move to another trait
    /// Writes an element in compressed form to the buffer.
    /*fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let compressed = self.to_bytes();
        writer.write_all(compressed.as_ref())
    }*/

    /// Returns the curve constant $a$.
    fn a() -> Self::Base;

    /// Returns the curve constant $b$.
    fn b() -> Self::Base;
}

/// The affine coordinates of a point on an elliptic curve.
#[derive(Clone, Copy, Debug, Default)]
pub struct Coordinates<C: CurveAffine> {
    pub(crate) x: C::Base,
    pub(crate) y: C::Base,
}

impl<C: CurveAffine> Coordinates<C> {
    /// Returns the x-coordinate.
    ///
    /// Equivalent to `Coordinates::u`.
    pub fn x(&self) -> &C::Base {
        &self.x
    }

    /// Returns the y-coordinate.
    ///
    /// Equivalent to `Coordinates::v`.
    pub fn y(&self) -> &C::Base {
        &self.y
    }

    /// Returns the u-coordinate.
    ///
    /// Equivalent to `Coordinates::x`.
    pub fn u(&self) -> &C::Base {
        &self.x
    }

    /// Returns the v-coordinate.
    ///
    /// Equivalent to `Coordinates::y`.
    pub fn v(&self) -> &C::Base {
        &self.y
    }
}

impl<C: CurveAffine> ConditionallySelectable for Coordinates<C> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Coordinates {
            x: C::Base::conditional_select(&a.x, &b.x, choice),
            y: C::Base::conditional_select(&a.y, &b.y, choice),
        }
    }
}
