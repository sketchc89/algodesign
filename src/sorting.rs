use std::fmt::Debug;

pub fn insertion_sort<T>(v: &mut [T])
    where T: PartialOrd + Debug {

    let len = v.len();
    if len < 2 {
        return;
    }
    for outer in 1..len {
        for inner in 0..outer {
            if inner >= outer {
                continue
            }
            if v[outer] < v[inner] {
                v.swap(outer, inner);
            }
        }
    }
}

// pub fn quick_sort<T>(v: &mut [T])

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![0_i32, 1, 2, -1, -2];
        insertion_sort(&mut v);
        assert_eq!(v, [-2_i32, -1, 0, 1, 2]);
    }

    #[test]
    fn test_insertion_sort_w_duplicates() {
        let mut v = [-3_i32, -3, -2, 10, -2, 0, 11];
        insertion_sort(&mut v);
        assert_eq!(v, [-3_i32, -3, -2, -2, 0, 10, 11]);
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut v : [i32; 0] = [];
        insertion_sort(&mut v);
        assert_eq!(v, []);
    }

    #[test]
    fn test_insertion_sort_float() {
        let mut v = [-1.0_f64, 1.0, 0.0];
        insertion_sort(&mut v);
        assert_eq!(v, [-1.0_f64, 0.0, 1.0]);
    }
}
