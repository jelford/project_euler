

pub fn is_product_of_3_or_5(n: &u64) -> bool {
    n % 3 == 0 || n % 5 == 0
}



#[cfg(test)]
mod tests {
    use super::is_product_of_3_or_5;

    #[test]
    fn is_product_of_3_or_5_true_for_multiples_of_three() {
        assert!(is_product_of_3_or_5(&3));
        assert!(is_product_of_3_or_5(&6));
        assert!(is_product_of_3_or_5(&9));
        assert!(is_product_of_3_or_5(&120));
        assert!(is_product_of_3_or_5(&9999));
    }

    #[test]
    fn is_product_of_3_or_5_true_for_multiples_of_five() {
        assert!(is_product_of_3_or_5(&5));
        assert!(is_product_of_3_or_5(&10));
        assert!(is_product_of_3_or_5(&15));
        assert!(is_product_of_3_or_5(&120));
        assert!(is_product_of_3_or_5(&10000));
    }


    #[test]
    fn is_product_of_3_or_5_false_for_multiples_of_neither() {
        assert!(!is_product_of_3_or_5(&7));
        assert!(!is_product_of_3_or_5(&11));
        assert!(!is_product_of_3_or_5(&13));
        assert!(!is_product_of_3_or_5(&121));
        assert!(!is_product_of_3_or_5(&10001));
    }
}
