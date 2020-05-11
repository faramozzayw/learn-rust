use reduce::Reduce;

#[derive(Debug)]
pub enum Specialty {
	Textile,
	Weaving,
	Cloth,
	Tobacco,
	Sewing,
	Pharmaceutical,
	Cotton,
}

#[derive(Debug)]
pub struct Factory {
	pub name: &'static str,
	pub specialty: Specialty,
	pub avg_salary: i32,
	pub avg_age: i32,
}

impl Factory {
	pub fn new(name: &'static str, specialty: Specialty, avg_salary: i32, avg_age: i32) -> Self {
		Self {
			name,
			specialty,
			avg_salary,
			avg_age,
		}
	}
}

impl Factory {
	pub fn calc_avg_salary(factories: &Vec<Factory>) -> i32 {
		let sum = factories
			.into_iter()
			.map(|factory| factory.avg_salary)
			.reduce(|a, b| a + b);

		sum.unwrap() / factories.len() as i32
	}
}