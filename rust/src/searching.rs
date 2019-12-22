use std::fmt::Debug;

/// Return index of element in an array if that element exists
///
/// # Notes
///
/// Requires input to be sorted
/// Will return the index of one of the elements rather than the first or last element.
///
/// # Examples
/// ```
/// use algodesign::searching::binary_search;
///
/// let v = vec![0, 1, 2, 5];
/// assert_eq!(binary_search(&v, &2), Some(2));
/// assert_eq!(binary_search(&v, &4), None);
/// ```
///
pub fn binary_search<T>(v: &[T], elem: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let len = v.len();
    binary_search_core(v, elem, 0, len)
}

/// Return index of an element
///
/// # Notes
///
/// lo and hi are a half open range
///
fn binary_search_core<T>(v: &[T], elem: &T, lo: usize, hi: usize) -> Option<usize>
where
    T: PartialOrd,
{
    let mid = (lo + hi) / 2;
    if v[mid] == *elem {
        Some(mid)
    } else if v[mid] > *elem && mid > lo {
        binary_search_core(v, elem, lo, mid)
    } else if v[mid] < *elem && (mid + 1) < hi {
        binary_search_core(v, elem, mid + 1, hi)
    } else {
        None
    }
}

/// Return index of first and last occurence of element in an array if that element exists
///
/// # Notes
///
/// Requires input to be sorted
/// The last element is in the range (i.e. the range would be inclusive
///
/// # Examples
/// ```
/// use algodesign::searching::binary_search_range;
///
/// let v = vec![0, 1, 2, 2, 2, 3, 5];
/// assert_eq!(binary_search_range(&v, &2), (Some(2), Some(4)));
/// assert_eq!(binary_search_range(&v, &4), (None, None));
/// ```
///
pub fn binary_search_range<T>(v: &[T], elem: &T) -> (Option<usize>, Option<usize>)
where
    T: PartialOrd + Debug,
{
    let len = v.len();
    (
        binary_search_first(v, elem, 0, len),
        binary_search_last(v, elem, 0, len),
    )
}

/// Return index of first element
///
/// # Notes
///
/// lo and hi are a half open range
///
fn binary_search_first<T>(v: &[T], elem: &T, lo: usize, hi: usize) -> Option<usize>
where
    T: PartialOrd + Debug,
{
    let mid = (lo + hi) / 2;

    if v[mid] == *elem {
        if mid == 0 {
            Some(mid)
        } else if v[mid - 1] < v[mid] {
            Some(mid)
        } else {
            binary_search_first(v, elem, lo, mid)
        }
    } else if v[mid] > *elem && mid > lo {
        binary_search_first(v, elem, lo, mid)
    } else if v[mid] < *elem && (mid + 1) < hi {
        binary_search_first(v, elem, mid + 1, hi)
    } else {
        println!(
            "Failed to find elem {:?} between {:?} and {:?}. {:?}",
            *elem, lo, hi, v[mid]
        );
        None
    }
}

/// Return index of last element
///
/// # Notes
///
/// lo and hi are a half open range
///
fn binary_search_last<T>(v: &[T], elem: &T, lo: usize, hi: usize) -> Option<usize>
where
    T: PartialOrd + Debug,
{
    let mid = (lo + hi) / 2;
    if v[mid] == *elem {
        if mid + 1 == hi {
            Some(mid)
        } else if v[mid + 1] > v[mid] {
            Some(mid)
        } else {
            binary_search_last(v, elem, mid + 1, hi)
        }
    } else if v[mid] > *elem && mid > lo {
        binary_search_last(v, elem, lo, mid)
    } else if v[mid] < *elem && (mid + 1) < hi {
        binary_search_last(v, elem, mid + 1, hi)
    } else {
        println!(
            "Failed to find last elem {:?} between {:?} and {:?}. {:?}",
            *elem, lo, hi, v[mid]
        );
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

    #[test]
    fn binary_search_first_last_none() {
        let v = vec![0, 0, 0, 1, 1, 1, 2, 3, 4, 5, 6, 6, 6, 6];
        let (first, last) = binary_search_range(&v, &10);
        assert_eq!(first, None);
        assert_eq!(last, None);
    }

    #[test]
    fn binary_search_first_last_elem() {
        let v = vec![0, 0, 0, 1, 1, 1, 2, 3, 4, 5, 6, 6, 6, 6];
        let (first, last) = binary_search_range(&v, &1);
        assert_eq!(first, Some(3));
        assert_eq!(last, Some(5));
    }

    #[test]
    fn binary_search_first_last_elem_all_same() {
        let v = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let (first, last) = binary_search_range(&v, &0);
        assert_eq!(first, Some(0));
        assert_eq!(last, Some(v.len() - 1));
    }

    #[test]
    fn binary_search_first_last_elem_all_same_none() {
        let v = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let (first, last) = binary_search_range(&v, &1);
        assert_eq!(first, None);
        assert_eq!(last, None);
    }

    #[test]
    fn binary_search_first_last_elem_all_same_diff() {
        let v = vec![0, 1, 2, 2, 2, 3, 5];
        assert_eq!(binary_search_range(&v, &2), (Some(2), Some(4)));
        assert_eq!(binary_search_range(&v, &4), (None, None));
    }
}
