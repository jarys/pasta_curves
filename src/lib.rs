//! Implementation of the Pallas / Vesta curve cycle.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "unboxed_closures", feature(unboxed_closures))]
#![cfg_attr(feature = "unboxed_closures", feature(fn_traits))]
#![allow(unknown_lints)]
#![allow(clippy::op_ref, clippy::same_item_push, clippy::upper_case_acronyms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(any(not(feature = "unboxed_closures"), feature = "sqrt-table"))]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod macros;
mod curves;
mod fields;

pub mod arithmetic;
pub mod pallas;
pub mod vesta;

#[cfg(feature = "unboxed_closures")]
mod hash_to_curve2;
mod hashtocurve;

pub use curves::*;
pub use fields::*;

pub extern crate group;

#[test]
fn test_endo_consistency() {
    use crate::arithmetic::{CurveExt, FieldExt};
    use group::Group;

    let a = pallas::Point::generator();
    assert_eq!(a * pallas::Scalar::ZETA, a.endo());
    let a = vesta::Point::generator();
    assert_eq!(a * vesta::Scalar::ZETA, a.endo());
}
