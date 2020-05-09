#[cfg(test)]
mod test {
	use crate::sort::shell_sort;

    #[test]
    fn it_sort_i32() {
        let ascending = |a, b| -> bool { a < b };

		// Sort empty vector
		let mut v: Vec<i32> = vec![];
		shell_sort(&mut v, ascending);
        assert_eq!(v, vec![]);
        
        let mut v = vec![0, 1, 2, 3, 4];
		shell_sort(&mut v, ascending);
		assert_eq!(v, vec![0, 1, 2, 3, 4]);

		let mut v = vec![0, 1, 2, 3, 4];
		v.reverse();
		shell_sort(&mut v, ascending);
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
        

        let mut v = vec![7, 6, 8, 9, 3, 2, 10, 5, 1];
		v.reverse();
		shell_sort(&mut v, ascending);
        assert_eq!(v, vec![1, 2, 3, 5, 6, 7, 8, 9, 10]);
        
        let descending = |a, b| -> bool { a > b };

		let mut v = vec![0, 1, 2, 3, 4];
		shell_sort(&mut v, descending);
		assert_eq!(v, vec![4, 3, 2, 1, 0]);
    }
}