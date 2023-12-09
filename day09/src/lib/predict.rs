// https://stackoverflow.com/a/65563202
fn count_combination(n: i64, r: i64) -> i64 {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn minus_pow(exp: i64) -> i64 {
    (-1i64).pow(exp as u32)
}

pub fn predict_next(a: &Vec<i64>) -> i64 {
    let n = a.len() as i64;

    // ru: https://vk.com/@mathemynka-ap2

    // subtraction between zeros after finding the arithmetic progression of the first order has no effect
    // and programming the stop condition is so lazy
    (0..n)
        .map(|k| {
            count_combination(n, k)
                * (0..=k)
                    .map(|i| minus_pow(i) * count_combination(k, i) * a[(k - i) as usize])
                    .sum::<i64>()
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_order_seq() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(&seq), 18);

        let seq_negative = vec![15, 12, 9, 6];
        assert_eq!(predict_next(&seq_negative), 3);
    }

    #[test]
    fn test_second_order_seq() {
        let seq = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(&seq), 28);

        let seq_negative = vec![28, 21, 15, 10, 6];
        assert_eq!(predict_next(&seq_negative), 3);
    }

    #[test]
    fn test_third_order_seq() {
        let seq = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&seq), 68);

        let seq_negative = vec![68, 45, 30, 21, 16];
        assert_eq!(predict_next(&seq_negative), 13);
    }
}
