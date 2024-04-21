use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Debug, Copy)]
pub struct VectorN<const N: usize> {
	pub coordinates: [f64; N],
}

impl<const N: usize> VectorN<N> {
	pub fn new(coordinates: [f64; N]) -> Self {
		return Self {
			coordinates
		};
	}
	pub fn clamp(&mut self, bounds: (f64, f64)) {
		for a in &mut self.coordinates {
			*a = a.clamp(bounds.0, bounds.1);
		}
	}
}

impl<const N: usize> Add<f64> for VectorN<N> {
	type Output = VectorN<N>;
	fn add(self, rhs: f64) -> Self::Output {
		return VectorN::<N> {
			coordinates: self.coordinates.map(|a| a + rhs),
		};
	}
}

impl<const N: usize> Mul<f64> for VectorN<N> {
	type Output = VectorN<N>;
	fn mul(self, rhs: f64) -> Self::Output {
		return VectorN::<N> {
			coordinates: self.coordinates.map(|a| a * rhs),
		};
	}
}

impl<const N: usize> Div<f64> for VectorN<N> {
	type Output = VectorN<N>;
	fn div(self, rhs: f64) -> Self::Output {
		return VectorN::<N> {
			coordinates: self.coordinates.map(|a| a / rhs),
		};
	}
}

impl<const N: usize> Mul for VectorN<N> {
	type Output = VectorN<N>;
	fn mul(mut self, rhs: Self) -> Self::Output {
		for index in 0..N {
			self.coordinates[index] *= rhs.coordinates[index];
		}
		return self;
	}
}

impl<const N: usize> Sub for VectorN<N> {
	type Output = VectorN<N>;

	fn sub(mut self, rhs: Self) -> Self::Output {
		for index in 0..N {
			self.coordinates[index] -= rhs.coordinates[index];
		}
		return self;
	}
}

impl<const N: usize> SubAssign for VectorN<N> {
	fn sub_assign(&mut self, rhs: Self) {
		for index in 0..N {
			self.coordinates[index] -= rhs.coordinates[index];
		}
	}
}

impl<const N: usize> Add for VectorN<N> {
	type Output = VectorN<N>;

	fn add(mut self, rhs: Self) -> Self::Output {
		for index in 0..N {
			self.coordinates[index] += rhs.coordinates[index];
		}
		return self;
	}
}

impl<const N: usize> AddAssign for VectorN<N> {
	fn add_assign(&mut self, rhs: Self) {
		for index in 0..N {
			self.coordinates[index] += rhs.coordinates[index];
		}
	}
}

impl <const N: usize> AddAssign<f64> for VectorN<N> {
	fn add_assign(&mut self, rhs: f64) {
		for index in 0..N {
			self.coordinates[index] += rhs;
		}
	}
}

impl<const N: usize> Default for VectorN<N> {
	fn default() -> Self {
		return Self {
			coordinates: [0.0; N],
		};
	}
}

pub trait QuickFold {
	fn sum(&self) -> f64;
	fn product(&self) -> f64;
	fn magnitude(&self) -> f64;
}

impl<const N: usize> QuickFold for [f64; N] {
	fn sum(&self) -> f64 {
		let mut result = 0.0;
		for entry in self {
			result += entry;
		}
		return result;
	}
	fn product(&self) -> f64 {
		let mut result = 1.0;
		for entry in self {
			result *= entry;
		}
		return result;
	}
	fn magnitude(&self) -> f64 {
		let mut result = 0.0;
		for entry in self {
			result += entry.powi(2);
		}
		return result.sqrt();
	}
}

#[cfg(test)]
mod test {
    use crate::vector::{QuickFold, VectorN};

	#[test]
	fn add_test() {
		let a = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let b = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let vecs_added = a + b;
		let f64_added = a + 1.0;
		let mut assign_added = a.clone();
		assign_added += b;
		let mut assign_added_f64 = a.clone();
		assign_added_f64 += 2.0;

		assert_eq!(vecs_added.coordinates, [2.0, 4.0, 6.0]);
		assert_eq!(f64_added.coordinates, [2.0, 3.0, 4.0]);
		assert_eq!(assign_added.coordinates, [2.0, 4.0, 6.0]);
		assert_eq!(assign_added_f64.coordinates, [3.0, 4.0, 5.0]);
	}

	#[test]
	fn sub_test() {
		let a = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let b = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let vecs_subbed = a - b;
		let mut subbed_assign = a.clone();
		subbed_assign -= b;
		assert_eq!(vecs_subbed.coordinates, [0.0, 0.0, 0.0]);
		assert_eq!(subbed_assign.coordinates, [0.0, 0.0, 0.0]);
	}

	#[test]
	fn mul_test() {
		let a = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let b = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		let vecs_mulled = a * b;
		let f64_mulled = a * 2.0;

		assert_eq!(vecs_mulled.coordinates, [1.0, 4.0, 9.0]);
		assert_eq!(f64_mulled.coordinates, [2.0, 4.0, 6.0]);
	}

	#[test]
	fn div_test() {
		let a = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};

		let divided = a / 2.0;

		assert_eq!(divided.coordinates, [0.5, 1.0, 1.5]);
	}

	#[test]
	fn clamp_test() {
		let mut a = VectorN::<_> {
			coordinates: [1.0, 2.0, 3.0]
		};
		a.clamp((1.5, 2.5));

		assert_eq!(a.coordinates, [1.5, 2.0, 2.5]);
	}
	#[test]
	fn sum_test() {
		let a = [1.0, 2.0, 3.0];
		assert_eq!(6.0, a.sum());
	}

	#[test]
	fn product_test() {
		let a = [2.0, 3.0, 4.0];
		assert_eq!(24.0, a.product());
	}
}