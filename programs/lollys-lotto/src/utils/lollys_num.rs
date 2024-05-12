use num_traits::{FromPrimitive, Num, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use std::fmt::{Debug, Display};

use crate::errors::{LollysLottoError, LollysLottoResult};

pub trait LollysNum:
    ToPrimitive + FromPrimitive + Num + Copy + Sized + PartialOrd + Debug + Display
{
    fn new(val: i64, scale: u32) -> Self;
    fn to_native(&self, scale: u32) -> Option<u64>;
    fn to_ui(amount: u64, scale: u32) -> Option<Self>;
    fn to_decimal(&self) -> LollysLottoResult<Decimal>;
    fn set_scale(&self, scale: u32) -> Option<Self>;
    fn sqrt(&self) -> Option<Self>;
}

impl LollysNum for f64 {
    fn new(val: i64, scale: u32) -> Self {
        let val = val as f64 / 10f64.powi(scale.try_into().unwrap());
        val
    }

    fn to_native(&self, scale: u32) -> Option<u64> {
        Some((*self * 10f64.powi(scale as i32)) as u64)
    }

    fn to_ui(amount: u64, scale: u32) -> Option<Self> {
        let amount = amount as f64 / 10f64.powi(scale.try_into().unwrap());
        Some(amount)
    }

    fn to_decimal(&self) -> LollysLottoResult<Decimal> {
        Decimal::from_f64_retain(*self).ok_or_else(|| {
            LollysLottoError::MathError.with_cause(format!("failed to convert to decimal: {self}"))
        })
    }

    fn set_scale(&self, scale: u32) -> Option<Self> {
        Some(self / 10f64.powi(scale.try_into().unwrap()))
    }

    fn sqrt(&self) -> Option<Self> {
        Some(f64::sqrt(*self))
    }
}

impl LollysNum for Decimal {
    fn new(val: i64, scale: u32) -> Self {
        Decimal::new(val, scale)
    }

    fn to_native(&self, scale: u32) -> Option<u64> {
        let mut this = *self;
        if this.scale() < scale {
            this.rescale(scale);
        }
        let this_scale = this.scale();
        let _ = Decimal::set_scale(&mut this, this_scale - scale).ok()?;
        this.to_u64()
    }

    fn to_ui(amount: u64, scale: u32) -> Option<Self> {
        let mut amount = Decimal::from_u64(amount)?;
        Decimal::set_scale(&mut amount, scale).ok()?;
        Some(amount)
    }

    fn to_decimal(&self) -> LollysLottoResult<Decimal> {
        Ok(*self)
    }

    fn set_scale(&self, scale: u32) -> Option<Self> {
        let mut amount = *self;
        Decimal::set_scale(&mut amount, scale).ok()?;
        Some(amount)
    }

    fn sqrt(&self) -> Option<Self> {
        MathematicalOps::sqrt(self)
    }
}

/// Convert between native u64 token amounts
/// and their [rust_decimal::Decimal] "UI" representations.
/// Native == Lamports
/// "UI" == SOL
pub mod token_amount {
    use rust_decimal::Decimal;

    use crate::errors::LollysLottoError;

    use super::LollysNum;

    /// Convert a [Decimal] to a native u64 amount.
    /// You must pass the mint's `decimals` as the scale.
    pub fn to_native<N>(amount: N, scale: u32) -> Result<u64, LollysLottoError>
    where
        N: LollysNum,
    {
        amount.to_native(scale).ok_or(LollysLottoError::MathError)
    }

    /// Convert a u64 value to a "UI" [Decimal] representation.
    /// You must pass the mint's `decimals` as the scale.
    pub fn to_ui<N>(amount: u64, scale: u32) -> Result<N, LollysLottoError>
    where
        N: LollysNum,
    {
        N::to_ui(amount, scale).ok_or(LollysLottoError::MathError)
    }

    /// Convert a u64 value to a "UI" [Decimal] representation.
    /// You must pass the mint's `decimals` as the scale.
    pub fn u128_to_ui(amount: u128, scale: u32) -> Decimal {
        let mut amount = Decimal::from(amount);
        Decimal::set_scale(&mut amount, scale).unwrap();

        amount
    }
}
