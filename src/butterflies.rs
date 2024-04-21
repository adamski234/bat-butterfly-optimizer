use rand::{distributions::{Distribution, Uniform}, prelude::SliceRandom, Rng};

use crate::vector::VectorN;

#[derive(Clone, Debug)]
pub struct Butterfly<const N: usize> {
    position: VectorN<N>,
    fragrance_multiplier: f64,
    fragrance_value: f64, // modification as per slide 15
    optimization_function: fn(VectorN<N>) -> f64,
    function_value: f64,
    function_bounds: (f64, f64),
}

impl<const N: usize> Butterfly<N> {
    fn new<RngType: Rng>(function_bounds: (f64, f64), optimization_function: fn(VectorN<N>) -> f64, fragrance_multiplier: f64, random_source: &mut RngType) -> Self {
        let mut coords_array = [0.0; N];

        let range = Uniform::from(function_bounds.0..function_bounds.1);

        coords_array.fill_with(|| range.sample(random_source));
        
        let position = VectorN::new(coords_array);
        let function_value = optimization_function(position);

        return Self {
            position, fragrance_multiplier,
            fragrance_value: function_value / (function_value + f64::EPSILON),
            function_bounds, optimization_function, function_value
        };
    }

    fn move_butterfly_global<RngType: Rng>(&mut self, best_butterfly_position: VectorN<N>, fragrance_exponent: f64, best_iter_solution: f64, random_source: &mut RngType) {
        self.position += (best_butterfly_position * random_source.gen::<f64>().powi(2) - self.position) * (self.fragrance_multiplier * self.fragrance_value.powf(fragrance_exponent));
        self.position.clamp(self.function_bounds);
        self.function_value = (self.optimization_function)(self.position);
        self.fragrance_value = self.function_value / (best_iter_solution + f64::EPSILON);
    }

    fn move_butterfly_local<RngType: Rng>(&mut self, random_butterfly_position_1: VectorN<N>, random_butterfly_position_2: VectorN<N>, fragrance_exponent: f64, best_iter_solution: f64, random_source: &mut RngType) {
        self.position += (random_butterfly_position_1 * random_source.gen::<f64>().powi(2) - random_butterfly_position_2) * (self.fragrance_multiplier * self.fragrance_value.powf(fragrance_exponent));
        self.position.clamp(self.function_bounds);
        self.function_value = (self.optimization_function)(self.position);
        self.fragrance_value = self.function_value / (best_iter_solution + f64::EPSILON);
    }

    fn reset<RngType: Rng>(&mut self, random_source: &mut RngType) {
        let range = Uniform::from(self.function_bounds.0..self.function_bounds.1);
        self.position.coordinates.fill_with(|| range.sample(random_source));
        self.function_value = (self.optimization_function)(self.position);
        self.fragrance_value = self.function_value / (self.function_value + f64::EPSILON);
    }
}

#[derive(Debug, Clone)]
pub struct WorldState<const N: usize, RngType: Rng> {
    population: Vec<Butterfly<N>>,
    pub best_solution: VectorN<N>,
    pub best_solution_value: f64,
    random_generator: RngType,
    fragrance_exponent_bounds: (f64, f64), // progresses with iterations
    local_search_chance: f64, // between 0 and 1
}

impl<const N: usize> WorldState<N, rand::rngs::StdRng> {
    pub fn new(pop_size: usize, function: fn(VectorN<N>) -> f64, bounds: (f64, f64), fragrance_multiplier: f64, fragrance_exponent_bounds: (f64, f64), local_search_chance: f64, mut random_source: rand::rngs::StdRng) -> Self {
        if bounds.0 >= bounds.1 {
            panic!("Incorrect order of bounds or zero size");
        }
        if fragrance_exponent_bounds.0 > fragrance_exponent_bounds.1 {
            panic!("Incorrect order of fragrance bounds");
        }

        let mut butterflies = Vec::with_capacity(pop_size);

        let mut best_solution = VectorN::default();
        let mut best_solution_value = f64::INFINITY;

        for _ in 0..pop_size {
            let butterfly = Butterfly::new(bounds, function, fragrance_multiplier, &mut random_source);
            if butterfly.function_value < best_solution_value {
                best_solution_value = butterfly.function_value;
                best_solution = butterfly.position;
            }
            butterflies.push(butterfly);
        }
        
        return Self {
            population: butterflies,
            best_solution, best_solution_value,
            random_generator: random_source,
            fragrance_exponent_bounds, local_search_chance
        };
    }

    pub fn reset(&mut self) {
        self.best_solution_value = f64::INFINITY;

        for butterfly in &mut self.population {
            butterfly.reset(&mut self.random_generator);
            if butterfly.function_value < self.best_solution_value {
                self.best_solution_value = butterfly.function_value;
                self.best_solution = butterfly.position;
            }
        }
    }

    pub fn do_iteration(&mut self, iteration_number: usize, iteration_count: usize) {
        let old_butterflies = self.population.clone();
        let best_butterfly_of_previous_iter = old_butterflies.iter().min_by(|first, second| first.function_value.partial_cmp(&second.function_value).unwrap()).unwrap();
        let exponent_value = self.fragrance_exponent_bounds.0 + (self.fragrance_exponent_bounds.1 - self.fragrance_exponent_bounds.0) * (iteration_number / iteration_count) as f64;
        for butterfly in &mut self.population {
            if self.random_generator.gen_bool(self.local_search_chance) {
                let first_butterfly = old_butterflies.choose(&mut self.random_generator).unwrap();
                let second_butterfly = old_butterflies.choose(&mut self.random_generator).unwrap();
                butterfly.move_butterfly_local(first_butterfly.position, second_butterfly.position, exponent_value, best_butterfly_of_previous_iter.function_value, &mut self.random_generator);
            } else {
                butterfly.move_butterfly_global(best_butterfly_of_previous_iter.position, exponent_value, best_butterfly_of_previous_iter.function_value, &mut self.random_generator);
            }
            if butterfly.function_value < self.best_solution_value {
                self.best_solution_value = butterfly.function_value;
                self.best_solution = butterfly.position;
            }
        }
    }

    pub fn do_all_iterations(&mut self, iteration_count: usize) {
        for iteration in 0..iteration_count {
            self.do_iteration(iteration, iteration_count);
        }
    }
}