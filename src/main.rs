#![feature(generic_arg_infer)]
#![allow(clippy::needless_return)]

use swarm_optimizers::{bats, butterflies, functions::Functions};

const FN_SIZE: usize = 20;

use std::ops::AddAssign;
use clap::{Parser, Subcommand};
use rand::{rngs::StdRng, thread_rng, SeedableRng};

#[derive(Parser, Clone, Debug)]
struct Config {
    #[arg(long = "functions", value_delimiter = ',', num_args = 1.., required = true)]
    functions: Vec<String>,

    #[arg(long = "try-count")]
    try_count: Option<usize>,
    
    #[command(subcommand)]
    command: OptimizationAlgorithmCommand,
}

#[derive(Subcommand, Clone, Debug)]
enum OptimizationAlgorithmCommand {
    Bats {
        #[arg(long = "bat-num-iters")]
        bat_num_iters: usize,

        #[arg(long = "bat-count")]
        bat_count: usize,
        
        #[arg(long = "frequency-left-bound")]
        frequency_left_bound: f64,

        #[arg(long = "frequency-right-bound")]
        frequency_right_bound: f64,
        
        #[arg(long = "initial-pulse-rate")]
        initial_pulse_rate: f64,

        #[arg(long = "pulse-rate-factor")]
        pulse_rate_factor: f64,

        #[arg(long = "initial-loudness")]
        initial_loudness: f64,

        #[arg(long = "loudness-cooling-rate")]
        loudness_cooling_rate: f64
    },

    Butterflies {
        #[arg(long = "butterfly-num-iters")]
        butterfly_num_iters: usize,

        #[arg(long = "butterfly-count")]
        butterfly_count: usize,

        #[arg(long = "fragrance-multiplier")]
        fragrance_multiplier: f64,

        #[arg(long = "fragrance-exponent-left-bound")]
        fragrance_exponent_left_bound: f64,

        #[arg(long = "fragrance-exponent-right-bound")]
        fragrance_exponent_right_bound: f64,

        #[arg(long = "local-search-chance")]
        local_search_chance: f64
    }
}

struct BatchRunData {
    pub min_result: f64,
    pub max_result: f64,
    pub average: f64,
    pub run_count: u32,
}

impl BatchRunData {
    fn new() -> Self {
        return Self {
            min_result: f64::MAX,
            max_result: f64::MIN,
            average: 0.0,
            run_count: 0,
        };
    }
}

impl AddAssign for BatchRunData {
    fn add_assign(&mut self, other: Self) {
        if other.max_result > self.max_result {
            self.max_result = other.max_result;
        }
        if other.min_result < self.min_result {
            self.min_result = other.min_result;
        }
        let self_sum = self.average * self.run_count as f64;
        let other_sum = other.average * other.run_count as f64;
        self.run_count += other.run_count;
        self.average = (self_sum + other_sum) / self.run_count as f64;
    }
}

impl AddAssign<f64> for BatchRunData {
    fn add_assign(&mut self, rhs: f64) {
        if rhs > self.max_result {
            self.max_result = rhs;
        }
        if rhs < self.min_result {
            self.min_result = rhs;
        }
        let previous_sum = self.average * self.run_count as f64;
        self.run_count += 1;
        self.average = (previous_sum + rhs) / self.run_count as f64;
        
    }
}

fn main() {
    let config = Config::parse();
    if config.functions.is_empty() {
        panic!("No functions given");
    }
    let test_functions = config.functions.into_iter().map(|s| {
        return (Functions::<FN_SIZE>::make_from_name(&s), s);
    }).collect::<Vec<_>>();

    if let Some(tries) = config.try_count {
        for (function, function_name) in test_functions {
            let bounds = function.get_bounds();
            let tries_per_thread = tries.div_ceil(num_cpus::get());
            let mut threads = Vec::with_capacity(num_cpus::get());
            
            match config.command {
                OptimizationAlgorithmCommand::Bats { bat_num_iters, 
                    bat_count, 
                    frequency_left_bound, 
                    frequency_right_bound, 
                    initial_pulse_rate, 
                    pulse_rate_factor, 
                    initial_loudness , 
                    loudness_cooling_rate
                } => {
                    let world = bats::WorldState::new(
                        bat_count,
                        function,
                        bounds,
                        (frequency_left_bound, frequency_right_bound),
                        initial_pulse_rate,
                        pulse_rate_factor,
                        initial_loudness, 
                        loudness_cooling_rate,
                        StdRng::from_rng(thread_rng()).unwrap()
                    );
                    for _ in 0..num_cpus::get() {
                        let mut thread_world = world.clone();
                        threads.push(std::thread::spawn(move || {
                            let mut run_stats = BatchRunData::new();
                            for _ in 0..tries_per_thread {
                                thread_world.do_all_iterations(bat_num_iters);
                                run_stats += function.calculate(thread_world.best_solution);
                                thread_world.reset();
                            }
                            return run_stats;
                        }));
                    }
                },

                OptimizationAlgorithmCommand::Butterflies { butterfly_num_iters, 
                    butterfly_count, 
                    fragrance_multiplier, 
                    fragrance_exponent_left_bound,
                    fragrance_exponent_right_bound, 
                    local_search_chance 
                } => {
                    let world = butterflies::WorldState::new(
                        butterfly_count,
                        function,
                        bounds,
                        fragrance_multiplier,
                        (fragrance_exponent_left_bound, fragrance_exponent_right_bound),
                        local_search_chance,
                        StdRng::from_rng(thread_rng()).unwrap()
                    );
                    for _ in 0..num_cpus::get() {
                        let mut thread_world = world.clone();
                        threads.push(std::thread::spawn(move || {
                            let mut run_stats = BatchRunData::new();
                            for _ in 0..tries_per_thread {
                                thread_world.do_all_iterations(butterfly_num_iters);
                                run_stats += function.calculate(thread_world.best_solution);
                                thread_world.reset();
                            }
                            return run_stats;
                        }));
                    }
                },
            }
            
            let result = threads.into_iter().map(|handle| handle.join().unwrap()).reduce(|mut a, b| {
                a += b;
                return a;
            }).unwrap();
            println!("{}: Finished {} runs. Max solution is {}. Average solution is {}. Min solution is {}.", function_name, result.run_count, result.max_result, result.average, result.min_result);
        }
    } else {
        let mut threads = Vec::new();
        for (function, function_name) in test_functions {
            let bounds = function.get_bounds();
            match config.command {
                OptimizationAlgorithmCommand::Bats { bat_num_iters, bat_count, frequency_left_bound, frequency_right_bound, initial_pulse_rate, pulse_rate_factor, initial_loudness, loudness_cooling_rate } => {
                    threads.push(std::thread::spawn(move || {
                        let mut world = bats::WorldState::new(
                            bat_count,
                            function,
                            bounds,
                            (frequency_left_bound, frequency_right_bound),
                            initial_pulse_rate,
                            pulse_rate_factor,
                            initial_loudness, 
                            loudness_cooling_rate,
                            StdRng::from_rng(thread_rng()).unwrap()
                        );
                        world.do_all_iterations(bat_num_iters);
                        println!("{}: Found optimum at {:?} = {}", function_name, world.best_solution.coordinates, function.calculate(world.best_solution));
                    }));
                },
                OptimizationAlgorithmCommand::Butterflies { 
                    butterfly_num_iters, 
                    butterfly_count, 
                    fragrance_multiplier, 
                    fragrance_exponent_left_bound,
                    fragrance_exponent_right_bound, 
                    local_search_chance 
                } => {
                    threads.push(std::thread::spawn(move || {
                        let mut world = butterflies::WorldState::new(
                            butterfly_count,
                            function,
                            bounds,
                            fragrance_multiplier,
                            (fragrance_exponent_left_bound, fragrance_exponent_right_bound),
                            local_search_chance,
                            StdRng::from_rng(thread_rng()).unwrap()
                        );
                        world.do_all_iterations(butterfly_num_iters);
                        println!("{}: Found optimum at {:?} = {}", function_name, world.best_solution.coordinates, function.calculate(world.best_solution));
                    }));
                },
            }
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
