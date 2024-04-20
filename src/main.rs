use bat_optimizer::{bats::WorldState, functions};

fn main() {
	let function_list = functions::create_function_list::<20>();
	let function_object = function_list.get("schwefel").unwrap();
	let mut world = WorldState::new(
		20, function_object.get_function(), function_object.get_bounds(), (0.0, 1.0),
		0.5, 0.1, 0.25, 0.1
	);
	world.do_all_iterations(1000000);
	/*world.do_iteration(0);
	println!("best solution after 0: {}", world.best_solution_value);
	world.do_iteration(1);
	println!("best solution after 1: {}", world.best_solution_value);*/
	eprintln!("1000 iters, solomon. Vector: {:?}. Solution: {}", world.best_solution.coordinates, world.best_solution_value);
}
