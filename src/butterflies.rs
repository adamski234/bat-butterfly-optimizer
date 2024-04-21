use rand::{distributions::{Distribution, Uniform}, rngs::ThreadRng, thread_rng, Rng};

use crate::vector::VectorN;

#[derive(Clone, Debug)]
pub struct Butterfly<const N: usize> {
    position: VectorN<N>,
    velocity: VectorN<N>,
    frequency_bounds: (f64, f64),
    original_pulse_rate: f64, // Should be between 0 and 1. Anything higher will be weird
    current_pulse_rate: f64, // Expresses the chance for a random walk using the loudness. Approaches original_pulse_rate
    pulse_rate_factor: f64,
    loudness: f64, // Loudness is the radius of random walk of the butterfly - similar to temperature in simulated annealing. Shrinks to 0.
    loudness_cool_factor: f64,
    best_solution_value: f64,
    bounds: (f64, f64),
}

impl<const N: usize> Butterfly<N> {
    fn new<RngType: Rng>(lower_bound: f64, upper_bound: f64, min_frequency: f64, max_frequency: f64, pulse_rate: f64, pulse_rate_factor: f64, loudness: f64, loudness_cool_factor: f64, random_source: &mut RngType) -> Self {
        let mut coords_array = [0.0; N];
        let mut speed_array = [0.0; N];

        let range = Uniform::from(lower_bound..upper_bound);

        coords_array.fill_with(|| { range.sample(random_source) });
        speed_array.fill_with(|| { random_source.gen::<f64>() });        

        return Self {
            position: VectorN::new(coords_array),
            velocity: VectorN::new(speed_array),
            current_pulse_rate: pulse_rate,
            original_pulse_rate: pulse_rate,
            frequency_bounds: (min_frequency, max_frequency),
            pulse_rate_factor, loudness, loudness_cool_factor,
            best_solution_value: f64::INFINITY,
            bounds: (lower_bound, upper_bound)
        };
    }

    fn move_butterfly<RngType: Rng>(&mut self, global_best_solution: VectorN<N>, random_source: &mut RngType, average_loudness: f64) {
        let frequency = random_source.gen_range(self.frequency_bounds.0..self.frequency_bounds.1);
        self.velocity += (self.position - global_best_solution) * frequency;
        self.position -= self.velocity; // According to all formulas this should be adding, not subtracting. However, adding produces awful results and makes butterflys divergent
        if random_source.gen::<f64>() < self.current_pulse_rate {
            self.position += random_source.gen_range(-1.0..1.0) * average_loudness;
        }
        self.position.clamp(self.bounds);
    }
    // Should only be called if the fitness improves
    fn update_parameters(&mut self, iteration_number: usize) {
        self.loudness *= self.loudness_cool_factor;
        self.current_pulse_rate = self.original_pulse_rate * (1.0 - (-self.pulse_rate_factor * iteration_number as f64).exp());
    }

    fn reset<RngType: Rng>(&mut self, lower_bound: f64, upper_bound: f64, pulse_rate: f64, loudness: f64, random_source: &mut RngType) {
        let range = Uniform::from(lower_bound..upper_bound);

        self.position.coordinates.fill_with(|| { range.sample(random_source) });
        self.velocity.coordinates.fill_with(|| { random_source.gen::<f64>() });
        self.best_solution_value = f64::INFINITY;
        self.current_pulse_rate = pulse_rate;
        self.original_pulse_rate = pulse_rate;
        self.loudness = loudness;
    }
}

#[derive(Debug, Clone)]
pub struct WorldState<const N: usize, RngType: Rng> {
    butterflys: Vec<Butterfly<N>>,
    function: fn(VectorN<N>) -> f64,
    pub best_solution: VectorN<N>,
    pub best_solution_value: f64,
    bounds: (f64, f64), // lower, upper
    random_generator: RngType,
    initial_pulse_rate: f64,
    initial_loudness: f64,
}

impl<const N: usize> WorldState<N, ThreadRng> {
    pub fn new(butterfly_count: usize, function: fn(VectorN<N>) -> f64, bounds: (f64, f64), frequency_bounds: (f64, f64), initial_pulse_rate: f64, pulse_rate_factor: f64, initial_loudness: f64, loudness_cool_factor: f64) -> Self {
        if bounds.0 >= bounds.1 {
            panic!("Incorrect order of bounds or zero size");
        }
        if frequency_bounds.0 >= frequency_bounds.1 {
            panic!("Incorrect order of frequency bounds or zero size");
        }

        let mut random_source = thread_rng();

        let mut butterflys = Vec::with_capacity(butterfly_count);
        for _ in 0..butterfly_count {
            butterflys.push(Butterfly::new(
                bounds.0, bounds.1, frequency_bounds.0, frequency_bounds.1,
                initial_pulse_rate, pulse_rate_factor, initial_loudness, loudness_cool_factor, &mut random_source,
            ));
        }

        let mut best_solution = VectorN::default();
        let mut best_solution_value = f64::INFINITY;
        for butterfly in &mut butterflys {
            let butterfly_value = function(butterfly.position);
            if butterfly_value < best_solution_value {
                best_solution = butterfly.position;
                best_solution_value = butterfly_value;
            }
        }

        return Self {
            butterflys, function, best_solution, best_solution_value, bounds,
            random_generator: random_source,
            initial_pulse_rate, initial_loudness,
        };
    }

    pub fn reset(&mut self) {
        self.best_solution = VectorN::default();
        self.best_solution_value = f64::INFINITY;
        for butterfly in &mut self.butterflys {
            butterfly.reset(self.bounds.0, self.bounds.1, self.initial_pulse_rate, self.initial_loudness, &mut self.random_generator);
            let butterfly_value = (self.function)(butterfly.position);
            if butterfly_value < self.best_solution_value {
                self.best_solution_value = butterfly_value;
                self.best_solution = butterfly.position;
            }
        }
    }
    
    pub fn move_butterflys(&mut self) {
        let average_loudness = self.butterflys.iter().map(|butterfly| butterfly.loudness).reduce(|acc, loudness| acc + loudness).unwrap() / (self.butterflys.len() as f64);
        for butterfly in &mut self.butterflys {
            butterfly.move_butterfly(self.best_solution, &mut self.random_generator, average_loudness);
        }
    }

    pub fn update_best_known_solution(&mut self, iter_number: usize) {
        for butterfly in &mut self.butterflys {
            let butterfly_value = (self.function)(butterfly.position);
            if butterfly_value < self.best_solution_value {
                self.best_solution_value = butterfly_value;
                self.best_solution = butterfly.position;
            }
            if butterfly_value < butterfly.best_solution_value {
                butterfly.best_solution_value = butterfly_value;
                butterfly.update_parameters(iter_number);
            }
        }
    }

    pub fn do_iteration(&mut self, iter_number: usize) {
        self.move_butterflys();
        self.update_best_known_solution(iter_number);
    }

    pub fn do_all_iterations(&mut self, iterations: usize) {
        for iter in 0..iterations {
            self.do_iteration(iter);
        }
    }
}