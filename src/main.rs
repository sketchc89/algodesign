mod sorting;
fn main() {
    println!("Sorting");
    let mut v = vec!(1i32, 0, 2, 1, -3, 3);

    println!("Before {:?}", v);
    sorting::insertion_sort(&mut v);
    println!("After  {:?}", v);

    v = vec![0_i32, 0, 0, 0, 0];
    let len = v.len();
    sorting::quick_sort(&mut v, 0, len - 1);
    assert_eq!(v, [-2_i32, -1, 0, 1, 2]);
}
