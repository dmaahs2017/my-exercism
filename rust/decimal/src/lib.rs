//! Arbitrary precision decimal arthimetic type
#![deny(missing_docs)]
#![feature(str_split_once)]


use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use std::cmp;
use std::fmt;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Eq, PartialEq, Debug)]
pub struct Decimal {
    numerator: BigInt,
    denominator: BigInt,
}

impl Decimal {
    /// Create new arbitrary-precision decimal.
    pub fn new(numerator: BigInt, denominator: BigInt) -> Self {
        let gcd = numerator.gcd(&denominator);
        Self {
            numerator: numerator / &gcd,
            denominator: denominator / &gcd,
        }
    }

    /// Create a new Decimal from a string. Supports positive and negative numbers.
    /// ```
    /// use decimal::Decimal;
    /// Decimal::try_from("-1.1").unwrap();
    /// ```
    pub fn try_from(input: &str) -> Option<Decimal> {
        if let Some(dec_pos) = input.find(".") {
            let mut numerator: BigInt = input[..dec_pos].parse().ok()?;
            let decimal_part = &input[dec_pos + 1..];

            let denominator = 10.to_bigint()?.pow(decimal_part.len() as u32);

            numerator = numerator * &denominator;
            if input.find("-").is_some() {
                numerator -= decimal_part.parse::<BigInt>().ok()?;
            } else {
                numerator += decimal_part.parse::<BigInt>().ok()?;
            }
            return Some(Self::new(numerator, denominator));
        }

        Some(Self::new(input.parse().ok()?, 1.to_bigint()?))
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = &self.numerator * &rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let denominator = &self.denominator * &rhs.denominator;
        let numerator = &self.numerator * rhs.denominator + &rhs.numerator * self.denominator;
        Self::new(numerator, denominator)
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let denominator = &self.denominator * &rhs.denominator;
        let numerator = &self.numerator * rhs.denominator - &rhs.numerator * self.denominator;
        Self::new(numerator, denominator)
    }
}

impl cmp::PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Decimal {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let lhs = &self.numerator * &other.denominator;
        let rhs = &other.numerator * &self.denominator;
        lhs.cmp(&rhs)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} / {}",
            self.numerator.to_string(),
            self.denominator.to_string()
        )
    }
}
