pub fn birthday_cake_candles(arr: &[i32]) -> i32 {
    let mut max = 0;
    let mut count = 0;

    for &candle in arr {
        if candle > max {
            max = candle;
            count = 1;
        } else if candle == max {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&arr), 2);
    }

    #[test]
    fn test_single() {
        let arr = vec![5];
        assert_eq!(birthday_cake_candles(&arr), 1);
    }
}