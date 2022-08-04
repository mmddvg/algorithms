pub mod Merge {

    pub fn merge_sort(mut arg: &[i32]) -> Vec<i32> {
        let len = arg.len();
        let mut res = Vec::new();
        if len == 1 {
            res.push(arg[0]);
            return res;
        }
        let middle = if len % 2 == 0 { len / 2 } else { (len + 1) / 2 };
        let (first, second) = (merge_sort(&arg[..middle]), merge_sort(&arg[middle..]));

        return sort(&first, &second);
    }

    fn sort(a: &[i32], b: &[i32]) -> Vec<i32> {
        let (a_len, b_len) = (a.len(), b.len());
        let mut res: Vec<i32> = Vec::with_capacity(a_len + b_len);
        let (mut i, mut j) = (0, 0);
        for z in 0..res.capacity() {
            if i == a_len {
                res.extend_from_slice(&b[j..]);
                return res;
            }
            if j == b_len {
                res.extend_from_slice(&a[i..]);
                return res;
            }
            if a[i] > b[j] {
                res.push(b[j]);
                j += 1;
            } else {
                res.push(a[i]);
                i += 1;
            }
        }
        return res;
    }
    #[test]
    fn merge_test() {
        let a = vec![42, 124, 532, 12, 93];
        let b = merge_sort(&a[..]);
        assert_eq!(vec![12, 42, 93, 124, 532], b);
    }

    #[test]
    fn sort_test() {
        let (a, b, c) = ([1, 3, 5], [2, 4], vec![1, 2, 3, 4, 5]);
        assert_eq!(sort(&a, &b), c);
    }
}
