use std::{
	clone::Clone,
	marker::Copy,
};

#[allow(dead_code)]
pub fn shell_sort<T, F>(vector: &mut Vec<T>, compare: F)
where 
T: PartialEq + PartialOrd + Clone + Copy,
F: Fn(T, T) -> bool
{
	let mut gap: i32 = (vector.len() / 2) as i32;

	while gap > 0 {
		let mut j: i32;
		let mut i: i32 = gap;

		while (i as usize) < vector.len()  {
			let value = vector[i as usize];

			j = i - gap;
			while 
				j >= 0 && !compare(vector[j as usize], value)
			{
				vector[(j + gap) as usize] = vector[j as usize];
				vector[j as usize] = value;

				j -= gap;
			}

			i += 1;
		}

		gap = gap / 2;
	}
}