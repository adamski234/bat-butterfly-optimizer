use rand::{distributions::{Distribution, Uniform}, rngs::ThreadRng, thread_rng, Rng};

use crate::vector::VectorN;

#[derive(Clone, Debug)]
pub struct Bat<const N: usize> {
    position: VectorN<N>,
    velocity: VectorN<N>,
    frequency_bounds: (f64, f64),
    original_pulse_rate: f64, // Should be between 0 and 1. Anything higher will be weird
    current_pulse_rate: f64, // Expresses the chance for a random walk using the loudness. Approaches original_pulse_rate
    pulse_rate_factor: f64,
    loudness: f64, // Loudness is the radius of random walk of the bat - similar to temperature in simulated annealing. Shrinks to 0.
    loudness_cool_factor: f64,
    best_solution_value: f64,
    bounds: (f64, f64),
}

impl<const N: usize> Bat<N> {
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

    fn move_bat<RngType: Rng>(&mut self, global_best_solution: VectorN<N>, random_source: &mut RngType, average_loudness: f64) {
        let frequency = random_source.gen_range(self.frequency_bounds.0..self.frequency_bounds.1);
        self.velocity += (global_best_solution - self.position) * frequency;
        self.position += self.velocity; // According to all formulas this should be adding, not subtracting. However, adding produces awful results and makes bats divergent
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
    bats: Vec<Bat<N>>,
    function: fn(VectorN<N>) -> f64,
    pub best_solution: VectorN<N>,
    pub best_solution_value: f64,
    bounds: (f64, f64), // lower, upper
    random_generator: RngType,
    initial_pulse_rate: f64,
    initial_loudness: f64,
}

impl<const N: usize> WorldState<N, ThreadRng> {
    pub fn new(bat_count: usize, function: fn(VectorN<N>) -> f64, bounds: (f64, f64), frequency_bounds: (f64, f64), initial_pulse_rate: f64, pulse_rate_factor: f64, initial_loudness: f64, loudness_cool_factor: f64) -> Self {
        if bounds.0 >= bounds.1 {
            panic!("Incorrect order of bounds or zero size");
        }
        if frequency_bounds.0 >= frequency_bounds.1 {
            panic!("Incorrect order of frequency bounds or zero size");
        }

        let mut random_source = thread_rng();

        let mut bats = Vec::with_capacity(bat_count);
        for _ in 0..bat_count {
            bats.push(Bat::new(
                bounds.0, bounds.1, frequency_bounds.0, frequency_bounds.1,
                initial_pulse_rate, pulse_rate_factor, initial_loudness, loudness_cool_factor, &mut random_source,
            ));
        }

        let mut best_solution = VectorN::default();
        let mut best_solution_value = f64::INFINITY;
        for bat in &mut bats {
            let bat_value = function(bat.position);
            if bat_value < best_solution_value {
                best_solution = bat.position;
                best_solution_value = bat_value;
            }
        }

        return Self {
            bats, function, best_solution, best_solution_value, bounds,
            random_generator: random_source,
            initial_pulse_rate, initial_loudness,
        };
    }

    pub fn reset(&mut self) {
        self.best_solution = VectorN::default();
        self.best_solution_value = f64::INFINITY;
        for bat in &mut self.bats {
            bat.reset(self.bounds.0, self.bounds.1, self.initial_pulse_rate, self.initial_loudness, &mut self.random_generator);
            let bat_value = (self.function)(bat.position);
            if bat_value < self.best_solution_value {
                self.best_solution_value = bat_value;
                self.best_solution = bat.position;
            }
        }
    }
    
    pub fn move_bats(&mut self) {
        let average_loudness = self.bats.iter().map(|bat| bat.loudness).reduce(|acc, loudness| acc + loudness).unwrap() / (self.bats.len() as f64);
        for bat in &mut self.bats {
            bat.move_bat(self.best_solution, &mut self.random_generator, average_loudness);
        }
    }

    pub fn update_best_known_solution(&mut self, iter_number: usize) {
        for bat in &mut self.bats {
            let bat_value = (self.function)(bat.position);
            if bat_value < self.best_solution_value {
                self.best_solution_value = bat_value;
                self.best_solution = bat.position;
            }
            if bat_value < bat.best_solution_value {
                bat.best_solution_value = bat_value;
                bat.update_parameters(iter_number);
            }
        }
    }

    pub fn do_iteration(&mut self, iter_number: usize) {
        self.move_bats();
        self.update_best_known_solution(iter_number);
    }

    pub fn do_all_iterations(&mut self, iterations: usize) {
        for iter in 0..iterations {
            self.do_iteration(iter);
        }
    }
}