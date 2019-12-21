use std::error::Error;

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

/// 4. O(a / b), iterates proportional to a and inversely prop to b
pub fn vi_4_div(a: i32, b: i32) -> Option<i32> {
    if b <= 0 {
        return None;
    }
    let mut count = 0;
    let mut sum = b;
    while sum <= a {
        sum += b;
        count += 1;
    }
    Some(count)
}

/// 5. O(lg2(n))
pub fn vi_5_sqrt(n: i32) -> Option<i32> {
    sqrt_helper(n, 1, n)
}

fn sqrt_helper(n: i32, min: i32, max: i32) -> Option<i32> {
    if max < min {
        return None;
    }
    let guess = (min + max) / 2;
    if guess * guess == n {
        Some(guess)
    } else if guess * guess < n {
        sqrt_helper(n, guess + 1, max)
    } else {
        sqrt_helper(n, min, guess - 1)
    }
}

/// 6. O(sqrt(n)), iterates through sqrt(n)
pub fn vi_6_sqrt(n: i32) -> Option<i32> {
    let mut guess = 1;
    while guess * guess <= n {
        if guess * guess == n {
            return Some(guess);
        }
        guess += 1;
    }
    None
}

/// 7. If bst isn't balanced it could take O(n) to check every element
/// 8. O(lg2(n))
/// 9. function is too horrible to refactor in rust, essentially clone, append
///    O(n) for appending,  O(1 + 2 + 3 + ... n) = O(n/2) for clone, so O(n^2)

/// 10. O(log10(n)). Proportional to number of digits which is base 10 log
pub fn vi_10_sum_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut leftover = n;
    while leftover > 0 {
        sum += leftover % 10;
        leftover /= 10;
    }
    sum
}

/// 11. O(N^2)
/// Checking if is_sorted takes O(N) worst case for already sorted strings which is common
/// Printing each string increases at O(N) ~26N

pub fn vi_11_sorted_strings(n: usize) {
    let s = String::new();
    print_sorted_strings(n, s);
}

fn print_sorted_strings(n: usize, prefix: String) {
    if n == 0 {
        if is_sorted_string(&prefix) {
            // println!("{:?}", prefix);
        }
    } else {
        for c in "abcdefghijklmnopqrtuvwxyz".chars() {
            let mut new_prefix = prefix.clone();
            new_prefix.push(c);
            print_sorted_strings(n - 1, new_prefix);
        }
    }
}

fn is_sorted_string(s: &String) -> bool {
    // Combine string with itself advanced one character and compare characters
    // e.g. "abcd".zip("bcd"), a > b,b > c,c > d;
    for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
        if c1 > c2 {
            return false;
        }
    }
    true
}

/// 12. O( b log(b) + a log(b) )
/// mergesort takes O(blg2(b)). binary search takes O(lg2(b)). bin search performed a times
// pub fn vi_12_intersection(a: Vec<i32>, b: Vec<i32>) -> i32 {
//     let mut intersect = 0;
//     for x in a {
//         if
// }

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
        assert_eq!(vi_4_div(13, 5), Some(2));
        assert_eq!(vi_4_div(12, 0), None);
    }

    #[test]
    fn vi_5() {
        assert_eq!(vi_5_sqrt(144), Some(12));
        assert_eq!(vi_5_sqrt(7), None);
        assert_eq!(vi_5_sqrt(-5), None);
    }

    #[test]
    fn vi_6() {
        assert_eq!(vi_6_sqrt(144), Some(12));
        assert_eq!(vi_6_sqrt(7), None);
        assert_eq!(vi_6_sqrt(-5), None);
    }

    #[test]
    fn vi_10() {
        assert_eq!(vi_10_sum_digits(1234), 10);
        assert_eq!(vi_10_sum_digits(-1234), 0);
        assert_eq!(vi_10_sum_digits(1001), 2);
        assert_eq!(vi_10_sum_digits(987), 24);
    }

    #[test]
    fn is_sorted_string_test() {
        assert!(is_sorted_string(&"abc".to_string()));
        assert!(is_sorted_string(&"aaaaabbcd".to_string()));
        assert!(is_sorted_string(&"".to_string()));
        assert!(!is_sorted_string(&"cba".to_string()));
        assert!(!is_sorted_string(&"cbabcab".to_string()));
    }

    #[test]
    fn vi_11() {
        vi_11_sorted_strings(3);
    }

    // fn vi_12() {
    //     assert_eq!(vi_12_intersection(vec![0, 1, 2, 3], vec![1, 3, 5, 6]), 2);
    //     assert_eq!(vi_12_intersection(vec![], vec![]), 0);
    // }
}
