use std::{f64::consts::{TAU, E}, collections::HashMap};

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

struct Ackley {}
impl<const N: usize> Function<N> for Ackley {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return ackley;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-32.0, 32.0);
	}
}

// functions 1
fn schwefel<const N: usize>(input: VectorN<N>) -> f64 {
	let absolutes = input.coordinates.map(f64::abs);
	return absolutes.map(|a| a.powi(2)).sum() + absolutes.product();
}

struct Schwefel {}
impl<const N: usize> Function<N> for Schwefel {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return schwefel;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-10.0, 10.0);
	}
}

// functions 1
fn brown<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| a.powi(2)).array_windows::<2>().map(|&[a, a_1]| {
		return a.powf(a_1 + 1.0) + a_1.powf(a + 1.0);
	}).sum();
}

struct Brown {}
impl<const N: usize> Function<N> for Brown {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return brown;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-1.0, 4.0);
	}
}

// functions 2
fn rastrigin<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| {
		return a.powi(2) - 10.0 * (TAU * a).cos() + 10.0;
	}).sum();
}

struct Rastrigin {}
impl<const N: usize> Function<N> for Rastrigin {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return rastrigin;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-5.12, 5.12);
	}
}

// functions 2
fn schwefel2<const N: usize>(input: VectorN<N>) -> f64 {
	return input.coordinates.map(|a| {
		return (a * a.abs().sqrt().sin()).abs();
	}).sum();
}

struct Schwefel2 {}
impl<const N: usize> Function<N> for Schwefel2 {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return schwefel2;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-100.0, 100.0);
	}
}

// functions 2
fn solomon<const N: usize>(input: VectorN<N>) -> f64 {
	let sum_of_squares = input.coordinates.map(|a| a.powi(2)).sum();
	return 1.0 - (TAU * sum_of_squares.sqrt()).cos() + 0.1 * sum_of_squares.sqrt();
}

struct Solomon {}
impl<const N: usize> Function<N> for Solomon {
	fn get_function(&self) -> fn(input: VectorN<N>) -> f64 {
		return solomon;
	}
	fn get_bounds(&self) -> (f64, f64) {
		return (-100.0, 100.0);
	}
}

pub fn create_function_list<const N: usize>() -> HashMap<String, Box<dyn Function<N>>> {
	let mut result: HashMap<String, Box<dyn Function<N>>> = HashMap::new();
	result.insert(String::from("ackley"), Box::new(Ackley {}));
	result.insert(String::from("schwefel"), Box::new(Schwefel {}));
	result.insert(String::from("brown"), Box::new(Brown {}));
	result.insert(String::from("rastrigin"), Box::new(Rastrigin {}));
	result.insert(String::from("schwefel2"), Box::new(Schwefel2 {}));
	result.insert(String::from("solomon"), Box::new(Solomon {}));
	return result;
}