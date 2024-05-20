use std::f64::consts::{E, TAU};

use crate::vector::VectorN;
use crate::vector::QuickFold;

pub trait Function<const N: usize> {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64;
	fn get_bounds(&self) -> (f64, f64);
}

// functions 1
fn ackley<const N: usize>(input: VectorN<N>) -> f64 {
	return -20.0 * (-0.2 * ((N as f64).recip() * input.coordinates.map(|a| a.powi(2)).sum()).sqrt()).exp() -
		((N as f64).recip() * input.coordinates.map(|a| (TAU * a).cos()).sum()).exp() +
		E + 20.0;
}

// functions 1
fn schwefel<const N: usize>(input: VectorN<N>) -> f64 {
	let absolutes = input.coordinates.map(f64::abs);
	return absolutes.map(|a| a.powi(2)).sum() + absolutes.product();
}

// functions 1
fn brown<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| a.powi(2)).array_windows::<2>().map(|&[a, a_1]| {
		return a.powf(a_1 + 1.0) + a_1.powf(a + 1.0);
	}).sum();
}

// functions 2
fn rastrigin<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| {
		return a.powi(2) - 10.0 * (TAU * a).cos() + 10.0;
	}).sum();
}

// functions 2
fn schwefel2<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| {
		return (a * a.abs().sqrt().sin()).abs();
	}).sum();
}

// functions 2
fn solomon<const N: usize>(input: VectorN<N>) -> f64 {
	let sum_of_squares = input.coordinates.map(|a| a.powi(2)).sum();
	return 1.0 - (TAU * sum_of_squares.sqrt()).cos() + 0.1 * sum_of_squares.sqrt();
}


#[derive(Debug, Clone, Copy)]
pub enum Functions<const N: usize> {
	Ackley,
	Schwefel,
	Brown,
	Rastrigin,
	Schwefel2,
	Solomon,
}

impl<const N: usize> Functions<N> {
	pub fn make_from_name(name: &str) -> Self {
		match name {
			"ackley" => return Self::Ackley,
			"schwefel" => return Self::Schwefel,
			"brown" => return Self::Brown,
			"rastrigin" => return Self::Rastrigin,
			"schwefel2" => return Self::Schwefel2,
			"solomon" => return Self::Solomon,
			_ => panic!("Nonexistent function passed: `{name}`"),
		}
	}

	pub fn get_bounds(self) -> (f64, f64) {
		match self {
			Self::Ackley => return (-32.0, 32.0),
			Self::Schwefel => return (-10.0, 10.0),
			Self::Brown => return (-1.0, 4.0),
			Self::Rastrigin => return (-5.12, 5.12),
			Self::Schwefel2 => return (-100.0, 100.0),
			Self::Solomon => return (-100.0, 100.0),
		}
	}

	pub fn calculate(self, input: VectorN<N>) -> f64 {
		match self {
			Functions::Ackley => return ackley(input),
			Functions::Schwefel => return schwefel(input),
			Functions::Brown => return brown(input),
			Functions::Rastrigin => return rastrigin(input),
			Functions::Schwefel2 => return schwefel2(input),
			Functions::Solomon => return solomon(input),
		}
	}
}