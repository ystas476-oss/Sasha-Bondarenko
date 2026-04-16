pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> Result<(), i32> {
    let total: i32 = bill.iter().sum();
    let actual = (total - bill[k]) / 2;

    if actual == b {
        Ok(())
    } else {
        Err(b - actual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overcharged() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 12), Err(5));
    }

    #[test]
    fn test_correct() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 7), Ok(()));
    }
}