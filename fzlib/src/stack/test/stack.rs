#[allow(unused_imports)]
use super::{
	Stack
};

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn it_basic_operation() {
        let mut s = Stack::new();

        s.push(1);
        s.push(2);

        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(2));

        assert_eq!(s.len(), 1);

        s.push(3);
        s.push(4);
        s.push(5);

        assert_eq!(s.len(), 4);
        assert_eq!(s.empty(), false);

        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.len(), 3);

        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.len(), 2);

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.len(), 1);

        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.len(), 0);

        assert_eq!(s.pop(), None);

        assert_eq!(s.empty(), true);


        let mut sv = Stack::from_vec(vec![1, 2, 3]);
        
        assert_eq!(sv.peek(), Some(1));
        
        sv.push(5);
        assert_eq!(sv.peek(), Some(5));

        assert_eq!(sv.pop(), Some(5));
        assert_eq!(sv.pop(), Some(1));
        assert_eq!(sv.pop(), Some(2));
    }

    #[test]
    fn it_peek() {
        let mut s = Stack::new();

        for i in 1..=10 {
            s.push(i);
            assert_eq!(s.peek(), Some(i));
        }

        s.clear();

        assert_eq!(s.peek(), None);
    }

    #[test]
    fn it_clear() {
        for i in 1..=30 {
            let mut s = Stack::new();

            for j in 1..=i {
                s.push(i * j);
            }

            assert_eq!(s.empty(), false);
            s.clear();

            assert_eq!(s.len(), 0);
            assert_eq!(s.peek(), None);
            assert_eq!(s.empty(), true);
        }
    }

    #[test]
    fn it_to_vec() {
        let a = Stack::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(a.to_vec(), vec![1, 2, 3, 4, 5]);

        let mut b = Stack::from_vec(vec![1, 2, 3]);
        b.push(4);
        b.push(5);
        assert_eq!(a.to_vec(), vec![1, 2, 3, 4, 5]);

        let mut c = Stack::new();
        c.push(1);
        c.push(2);
        c.push(3);
        assert_eq!(c.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn it_reverse() {
        let mut a = Stack::from_vec(vec![1, 2, 3, 4, 5]);
        let a_len = a.len();

        a.reverse();
        assert_eq!(a.len(), a_len);
        assert_eq!(a.to_vec(), vec![5, 4, 3, 2, 1]);

        let mut b = Stack::new();
        b.push(1);
        b.push(2);
        b.push(3);
        // b: [3, 2, 1]
        let b_len = b.len();
        
        b.reverse();
        assert_eq!(b.len(), b_len);
        assert_eq!(b.to_vec(), vec![1, 2, 3]);

        let mut c = Stack::from_vec(vec![1, 2, 3, 4, 5]);

        c.pop();
        c.pop();
        c.push(0);

        let c_len = c.len();

        c.reverse();
        assert_eq!(c.len(), c_len);
        assert_eq!(c.to_vec(), vec![5, 4, 3, 0]);
    }

    #[test]
    fn it_partial_eq() {
        let s1 = Stack::from_vec(vec![1, 2, 3]);
        let s2 = Stack::from_vec(vec![1, 2, 3]);

        assert_eq!(s1 == s2 , true);

        let s1 = Stack::from_vec(vec![1, 2, 3]);
        let s2 = Stack::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(s1 == s2 , false);

        let mut s1 = Stack::new();
        s1.push("a");
        s1.push("b");
        s1.push("c");

        let mut s2 = Stack::new();
        s2.push("a");
        s2.push("b");
        s2.push("C");

        assert_eq!(s1 == s2, false);

        let s1 = Stack::from_vec(vec![true, false]);
        let mut s2 = Stack::new();
        s2.push(false);
        s2.push(true);

        assert_eq!(s1 == s2, true);
    }
}
