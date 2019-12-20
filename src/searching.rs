use std::fmt::Debug;

pub fn binary_search<T>(_v: &[T], elem: T) -> Option<T>
where
    T: PartialOrd + Debug,
{
    println!("{:?}", elem);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let v = vec![2, 4, 10, 12, 15, 100];
        let res = binary_search(&v, 5);
        assert_eq!(res, None);
    }
}
