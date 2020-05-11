//use fzlib::prelude::*;
mod factory;

use factory::*;

fn main() {
	let factories = vec![
		Factory::new("A1", Specialty::Cloth, 12_000, 34),
		Factory::new("B1", Specialty::Pharmaceutical, 15_000, 32),
		Factory::new("C1", Specialty::Cotton, 8_000, 47),
		Factory::new("D1", Specialty::Weaving, 10_000, 45),
		Factory::new("E1", Specialty::Sewing, 7_500, 25),
		Factory::new("A2", Specialty::Cloth, 13_570, 27),
		Factory::new("C3", Specialty::Cloth, 15_010, 60),
		Factory::new("D4", Specialty::Tobacco, 9_760, 51),
	];

	println!("{:#?}", &factories);
	let avg = Factory::calc_avg_salary(&factories);
	println!("avg salary of all: \t{:#?}", avg);


	let count = &factories
		.iter()
		.map(|factory| factory.avg_salary)
		.filter(|a| *a >= avg)
		.collect::<Vec<_>>()
		.len();

	let mut res_vec = Vec::with_capacity(factories.len());
	
	for elem in factories.iter() {
		if elem.avg_salary >= avg {
			res_vec.push(elem);
		}
	};

	println!("count {:#?}", &count);
	println!("{:#?}", &res_vec);
}