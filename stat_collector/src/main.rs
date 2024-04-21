fn main() {
	let header = "fragrance_multiplier,local_search_chance,fn_name,max_solution,avg_solution,min_solution";

	let filename_gex = regex::Regex::new(r".*_(\d\.\d)_.*_(\d\.\d)").unwrap();
	let gex = regex::Regex::new(r"(.*): .*is (-?\d*(?:\.\d*)?)\..*is (-?\d*(?:\.\d*)?)\. .*is (-?\d*(?:\.\d*)?)").unwrap();

	println!("{}", header);

	for filename in glob::glob("./output_butterflies/*").unwrap() {
		let filename = filename.unwrap();
		let filename_captures = filename_gex.captures(filename.to_str().unwrap()).unwrap();
		let stat_data = std::fs::read_to_string(&filename).unwrap().lines().map(|line| {
			let captures = gex.captures(line).unwrap();
			//                                         fn_name       max_sol       avg_sol       min_sol
			return format!("{},{},{},{},{},{}", &filename_captures[2], &filename_captures[1], &captures[1], &captures[2], &captures[3], &captures[4]);
		}).collect::<Vec<_>>().join("\n");
		println!("{}", stat_data);
	}
}