use crate::{
    arithmetic::{CurveExt, FieldExt},
    curves::CurveConstants,
};
use core::marker::PhantomData;
use ff::Field;

#[derive(Debug, Copy, Clone)]
pub struct Hasher<'a, Field, Curve, IsoCurve> {
    domain_prefix: &'a str,
    _marker1: PhantomData<Field>,
    _marker2: PhantomData<Curve>,
    _marker3: PhantomData<IsoCurve>,
}

impl<'a, F, C, I> Hasher<'a, F, C, I> {
    pub(crate) fn new(domain_prefix: &'a str) -> Self {
        Hasher {
            domain_prefix: domain_prefix,
            _marker1: PhantomData,
            _marker2: PhantomData,
            _marker3: PhantomData,
        }
    }
}

impl<'a, F, C, I> Fn<(&[u8],)> for Hasher<'a, F, C, I>
where
    F: FieldExt,
    C: CurveExt<Base = F> + CurveConstants,
    I: CurveExt<Base = F>,
{
    extern "rust-call" fn call(&self, args: (&[u8],)) -> C {
        let (message,) = args;
        use crate::hashtocurve;
        let mut us = [Field::zero(); 2];
        hashtocurve::hash_to_field(C::CURVE_ID, self.domain_prefix, message, &mut us);
        let q0 = hashtocurve::map_to_curve_simple_swu::<F, C, I>(&us[0], C::THETA, C::Z);
        let q1 = hashtocurve::map_to_curve_simple_swu::<F, C, I>(&us[1], C::THETA, C::Z);
        let r = q0 + &q1;
        debug_assert!(bool::from(r.is_on_curve()));
        hashtocurve::iso_map::<F, C, I>(&r, &C::ISOGENY_CONSTANTS)
    }
}

impl<'a, F, C, I> FnOnce<(&[u8],)> for Hasher<'a, F, C, I>
where
    F: FieldExt,
    C: CurveExt<Base = F> + CurveConstants,
    I: CurveExt<Base = F>,
{
    type Output = C;
    extern "rust-call" fn call_once(self, args: (&[u8],)) -> C {
        self.call(args)
    }
}

impl<'a, F, C, I> FnMut<(&[u8],)> for Hasher<'a, F, C, I>
where
    F: FieldExt,
    C: CurveExt<Base = F> + CurveConstants,
    I: CurveExt<Base = F>,
{
    extern "rust-call" fn call_mut(&mut self, args: (&[u8],)) -> C {
        self.call(args)
    }
}

/*
impl<'a, F: FieldExt, C: CurveExt<Base = F> + CurveConstants, I: CurveExt<Base = F>>
    Hasher<'a, F, C, I>
{
    fn hash(&self, message: &[u8]) -> C {
        use crate::hashtocurve;
        let mut us = [Field::zero(); 2];
        hashtocurve::hash_to_field(C::CURVE_ID, self.domain_prefix, message, &mut us);
        let q0 = hashtocurve::map_to_curve_simple_swu::<F, C, I>(&us[0], C::THETA, C::Z);
        let q1 = hashtocurve::map_to_curve_simple_swu::<F, C, I>(&us[1], C::THETA, C::Z);
        let r = q0 + &q1;
        debug_assert!(bool::from(r.is_on_curve()));
        hashtocurve::iso_map::<F, C, I>(&r, &C::ISOGENY_CONSTANTS)
    }
}
*/
