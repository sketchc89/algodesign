/// Additional problems
/// VI.
/// 1. O(b), addition once for every b

pub fn vi_1_product(a: i32, b: i32) -> i32 {
    let mut sum: i32 = 0;
    for _i in 0..b {
        sum += a;
    }
    sum
}

/// 2. O(b), function called once for every b

pub fn vi_2_power(a: i32, b: i32) -> Option<i32> {
    if b < 0 {
        None
    } else if b == 0 {
        Some(1)
    } else {
        Some(a * vi_2_power(a, b - 1).unwrap())
    }
}

/// 3. O(1), only two constant time operations called
pub fn vi_3_mod(a: i32, b: i32) -> Option<i32> {
    if b <= 0 {
        None
    } else {
        let div = a / b;
        Some(a - div * b)
    }
}

/// 4.
pub fn vi_4_div(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let mut sum = b;
    while sum <= a {
        sum += b;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vi_1() {
        assert_eq!(vi_1_product(3, 4), 12);
    }

    #[test]
    fn vi_2() {
        assert_eq!(vi_2_power(3, 4), Some(81));
        assert_eq!(vi_2_power(3, -2), None);
    }

    #[test]
    fn vi_3() {
        assert_eq!(vi_3_mod(13, 5), Some(3));
        assert_eq!(vi_3_mod(12, 0), None);
    }

    #[test]
    fn vi_4() {
        assert_eq!(vi_4_div(13, 5), 2);
        assert_eq!(vi_4_div(12, 0), 0);
    }
}
