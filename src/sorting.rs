pub fn insertion_sort<T>(v: &mut [T])
where
    T: PartialOrd,
{
    let len = v.len();
    if len < 2 {
        return;
    }
    for outer in 1..len {
        for inner in 0..outer {
            if inner >= outer {
                continue;
            }
            if v[outer] < v[inner] {
                v.swap(outer, inner);
            }
        }
    }
}

pub fn quick_sort<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    q_sort(v, cmp, 0, len);
}

fn q_sort<T, F>(v: &mut [T], cmp: &F, lo: usize, hi: usize)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    if len > 1 {
        let pivot = q_partition(v, cmp, lo, hi);
        if pivot > 1 {
            q_sort(v, cmp, 0, pivot - 1);
        }
        if pivot + 1 < hi {
            q_sort(v, cmp, pivot + 1, hi);
        }
    }
}

fn q_partition<T, F>(v: &mut [T], cmp: &F, lo: usize, hi: usize) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let mut pivot_index = lo;
    for i in lo..hi {
        if cmp(&v[i], &v[lo]) {
            pivot_index += 1;
            v.swap(pivot_index, i);
        }
    }
    v.swap(lo, pivot_index);
    pivot_index
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
        let mut v: [i32; 0] = [];
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
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, [-2_i32, -1, 0, 1, 2]);
    }

    #[test]
    fn test_quick_sort_same() {
        let mut v = vec![0_i32, 0, 0, 0, 0];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, [0_i32, 0, 0, 0, 0]);
    }

    #[test]
    fn test_quick_sort_one() {
        let mut v = [1_u32];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, [1_u32]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut v: [i32; 0] = [];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, []);
    }
}
