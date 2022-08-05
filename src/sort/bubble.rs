pub fn bubble_sort(arg: &mut [i32]) {
    let mut last = arg.len();
    for i in 0..arg.len() {
        for j in 1..last {
            if arg[j - 1] > arg[j] {
                arg.swap(j - 1, j);
            }
        }
    }
}

#[test]
fn bubble_test() {
    let mut a = [5, 4, 3, 2, 1];
    bubble_sort(&mut a);
    assert_eq!(a, [1, 2, 3, 4, 5]);
}
