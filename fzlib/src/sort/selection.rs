use std::{
	clone::Clone,
	marker::Copy,
};

pub fn selection_sort<T, F>(vector: &mut Vec<T>, compare: F)
where 
T: PartialEq + PartialOrd + Clone + Copy,
F: Fn(T, T) -> bool
{
	let mut i: usize = 0;

	while i < vector.len() {
		let mut min = i;
		
		let mut j: usize = i + 1;
		while j < vector.len() {
			if compare(vector[j], vector[min]) {
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