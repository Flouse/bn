extern crate rand;
extern crate rustc_serialize;
extern crate byteorder;

mod arith;
mod fields;
mod groups;

use fields::FieldElement;
use groups::GroupElement;

use std::ops::{Add, Sub, Mul, Neg};
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Fr(fields::Fr);

impl Fr {
    pub fn zero() -> Self { Fr(fields::Fr::zero()) }
    pub fn one() -> Self { Fr(fields::Fr::one()) }
    pub fn random<R: Rng>(rng: &mut R) -> Self { Fr(fields::Fr::random(rng)) }
    pub fn pow(&self, exp: Fr) -> Self { Fr(self.0.pow(exp.0)) }
    pub fn from_str(s: &str) -> Option<Self> { fields::Fr::from_str(s).map(|e| Fr(e)) }
    pub fn inverse(&self) -> Option<Self> { self.0.inverse().map(|e| Fr(e)) }
}

impl Add<Fr> for Fr {
    type Output = Fr;

    fn add(self, other: Fr) -> Fr { Fr(self.0 + other.0) }
}

impl Sub<Fr> for Fr {
    type Output = Fr;

    fn sub(self, other: Fr) -> Fr { Fr(self.0 - other.0) }
}

impl Neg for Fr {
    type Output = Fr;

    fn neg(self) -> Fr { Fr(-self.0) }
}

impl Mul for Fr {
    type Output = Fr;

    fn mul(self, other: Fr) -> Fr { Fr(self.0 * other.0) }
}

#[derive(Copy, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct G1(groups::G1);

impl G1 {
    pub fn zero() -> Self { G1(groups::G1::zero()) }
    pub fn one() -> Self { G1(groups::G1::one()) }
    pub fn random<R: Rng>(rng: &mut R) -> Self { G1(groups::G1::random(rng)) }
}

impl Add<G1> for G1 {
    type Output = G1;

    fn add(self, other: G1) -> G1 { G1(self.0 + other.0) }
}

impl Sub<G1> for G1 {
    type Output = G1;

    fn sub(self, other: G1) -> G1 { G1(self.0 - other.0) }
}

impl Neg for G1 {
    type Output = G1;

    fn neg(self) -> G1 { G1(-self.0) }
}

impl Mul<Fr> for G1 {
    type Output = G1;

    fn mul(self, other: Fr) -> G1 { G1(self.0 * other.0) }
}

#[derive(Copy, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct G2(groups::G2);

impl G2 {
    pub fn zero() -> Self { G2(groups::G2::zero()) }
    pub fn one() -> Self { G2(groups::G2::one()) }
    pub fn random<R: Rng>(rng: &mut R) -> Self { G2(groups::G2::random(rng)) }
}

impl Add<G2> for G2 {
    type Output = G2;

    fn add(self, other: G2) -> G2 { G2(self.0 + other.0) }
}

impl Sub<G2> for G2 {
    type Output = G2;

    fn sub(self, other: G2) -> G2 { G2(self.0 - other.0) }
}

impl Neg for G2 {
    type Output = G2;

    fn neg(self) -> G2 { G2(-self.0) }
}

impl Mul<Fr> for G2 {
    type Output = G2;

    fn mul(self, other: Fr) -> G2 { G2(self.0 * other.0) }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Gt(fields::Fq12);

impl Gt {
    pub fn one() -> Self { Gt(fields::Fq12::one()) }
    pub fn pow(&self, exp: Fr) -> Self { Gt(self.0.pow(exp.0)) }
    pub fn inverse(&self) -> Self { Gt(self.0.inverse().unwrap()) }
}

impl Mul<Gt> for Gt {
    type Output = Gt;

    fn mul(self, other: Gt) -> Gt { Gt(self.0 * other.0) }
}

pub fn pairing(p: G1, q: G2) -> Gt {
    Gt(groups::pairing(&p.0, &q.0))
}
