pub fn binary_search<T>(v: &[T], elem: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let len = v.len();
    binary_search_core(v, elem, 0, len)
}

fn binary_search_core<T>(v: &[T], elem: &T, lo: usize, hi: usize) -> Option<usize>
where
    T: PartialOrd,
{
    let mid = (lo + hi) / 2;
    if v[mid] == *elem {
        Some(mid)
    } else if v[mid] > *elem && mid > lo {
        binary_search_core(v, elem, 0, mid - 1)
    } else if v[mid] < *elem && mid < hi {
        binary_search_core(v, elem, mid + 1, hi)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_not_found() {
        let v = vec![2, 4, 10, 12, 15, 100];
        let res = binary_search(&v, &5);
        assert_eq!(res, None);
    }

    #[test]
    fn binary_search_found() {
        let v = vec![2, 4, 10, 12, 15, 100];
        let res = binary_search(&v, &4);
        assert_eq!(res, Some(1));
    }

    #[test]
    fn binary_search_duplicates_nonspecific() {
        let v = vec![0, 0, 0, 1, 1];
        let res = binary_search(&v, &0);
        assert!(res.unwrap() < 3); // binary search doesn't specify whether it's the first, last, etc
    }
}
