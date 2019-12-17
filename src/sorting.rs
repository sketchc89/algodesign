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

pub fn quick_sort<T>(v: &mut [T], lo: usize, hi: usize)
    where T: PartialOrd + Copy + Debug {

    println!("{:?} {:?}", lo, hi);
    if lo < hi {
        let p_index = q_partition(v, lo, hi);
        quick_sort(v, lo, p_index);
        quick_sort(v, p_index + 1, hi);
    }
}

fn q_partition<T>(v: &mut [T], mut lo: usize, mut hi: usize) -> usize
    where T: PartialOrd + Copy + Debug {
    let pivot = v[lo + (hi - lo) / 2];
    loop {
    println!("{:?} {:?}", lo, hi);
        while lo > v.len() || v[lo] < pivot {
            lo += 1;
        }
        while v[hi] > pivot {
            if hi == 0 {
                break;
            }
            hi -= 1;
        }
        if lo >= hi {
            return hi;
        }
        v.swap(lo, hi);
        lo += 1;
        hi -= 1;
    }
}

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

    #[test]
    fn test_quick_sort() {
        let mut v = vec![0_i32, 1, 2, -1, -2];
        let len = v.len();
        quick_sort(&mut v, 0, len - 1);
        assert_eq!(v, [-2_i32, -1, 0, 1, 2]);
    }

    #[test]
    fn test_quick_sort_same() {
        let mut v = vec![0_i32, 0, 0, 0, 0];
        let len = v.len();
        quick_sort(&mut v, 0, len - 1);
        assert_eq!(v, [0_i32, 0, 0, 0, 0]);
    }
}
