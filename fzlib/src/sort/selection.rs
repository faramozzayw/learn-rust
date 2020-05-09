use std::{
	clone::Clone,
	marker::Copy,
};

pub fn selection_sort<T>(vector: &mut Vec<T>)
where T: PartialEq + PartialOrd + Clone + Copy
{
	let mut i: usize = 0;

	while i < vector.len() {
		let mut min = i;
		
		let mut j: usize = i + 1;
		while j < vector.len() {
			if vector[j] < vector[min] {
				min = j;
			}

			j += 1;
		}

		let dummy = vector[i];
		vector[i] = vector[min];
		vector[min] = dummy;

		i += 1;
	}
}