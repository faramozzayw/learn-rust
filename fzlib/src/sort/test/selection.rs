#[cfg(test)]
mod test {
	use crate::selection_sort;

	#[test]
	fn sort_i32() {
		let ascending = |a, b| -> bool { a < b };

		// Sort empty vector
		let mut v: Vec<i32> = vec![];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![]);

		let mut v = vec![0, 1, 2, 3, 4];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![0, 1, 2, 3, 4]);

		let mut v = vec![0, 1, 2, 3, 4];
		v.reverse();
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![0, 1, 2, 3, 4]);

		let mut v = vec![5, -1, -22, 345, 4];
		v.reverse();
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![-22, -1, 4, 5, 345]);

		let mut v = vec![5, -1, -22, 345, 4];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![-22, -1, 4, 5, 345]);

		let descending = |a, b| -> bool { a > b };

		let mut v = vec![0, 1, 2, 3, 4];
		selection_sort(&mut v, descending);
		assert_eq!(v, vec![4, 3, 2, 1, 0]);
	}

	#[test]
	fn sort_f32() {
		let ascending = |a, b| -> bool { a < b };

		// Sort empty vector
		let mut v: Vec<f32> = vec![];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![]);

		let mut v: Vec<f32> = vec![0.0];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![0.0]);

		let mut v = vec![0.234, -2.34, 2.57, 97.124, 40.25];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![-2.34, 0.234, 2.57, 40.25, 97.124]);
		
		let descending = |a, b| -> bool { a > b };

		let mut v = vec![1.1, 2.2, -4.4, -7.5, 10.3];
		selection_sort(&mut v, descending);
		assert_eq!(v, vec![10.3, 2.2, 1.1, -4.4, -7.5]);
	}

	#[test]
	fn sort_bool_why_not() {
		let ascending = |a, b| -> bool { a < b };

		let mut v = vec![true, false, false, true];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![false, false, true, true]);

		let descending = |a, b| -> bool { a > b };

		let mut v = vec![true, false, false, true];
		selection_sort(&mut v, descending);
		assert_eq!(v, vec![true, true, false, false]);
	}

	#[test]
	#[should_panic]
	fn panic_sort_i32() {
		let ascending = |a, b| -> bool { a < b };

		let mut v = vec![1, 1, 2, 3, 4];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![0, 1, 2, 3, 4]);


		let mut v = vec![0, -1, 1];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![-1, 1]);
	}

	#[test]
	#[should_panic]
	fn panic_sort_f32() {
		let ascending = |a, b| -> bool { a < b };

		let mut v = vec![0.234, -2.34, 2.57, 97.124, 40.25];
		selection_sort(&mut v, ascending);
		assert_eq!(v, vec![0.234, -2.34, 2.57, 97.124, 40.25]);
	}
}