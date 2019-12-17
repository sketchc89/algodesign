mod sorting;
fn main() {
    println!("Sorting");
    let mut v = vec!(1i32, 0, 2, 1, -3, 3);

    println!("Before {:?}", v);
    sorting::insertion_sort(&mut v);
    println!("After  {:?}", v);
}
