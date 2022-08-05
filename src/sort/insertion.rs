pub fn insertion_sort(arg: &mut [i32]) {
    for i in 1..(arg.len()) {
        let item = arg[i];
        let mut j = i - 1;

        while arg[j] > item {
            arg.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

#[test]
fn insertion_test() {
    let mut a = [52, 12, 64, 123, 14, 54];
    insertion_sort(&mut a);
    println!("{:?}", a);
    assert_eq!(a, [12, 14, 52, 54, 64, 123]);
}
