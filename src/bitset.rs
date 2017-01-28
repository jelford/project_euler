
pub fn powerset<T: Clone>(target: &Vec<T>) -> Vec<Vec<T>> {
    let top = 2_u64.pow(target.len() as u32);
    let mut res = Vec::with_capacity(top as usize);
    for i in 0..top {
        let mut this_set: Vec<T> = Vec::with_capacity(target.len() as usize);
        for (index, content) in target.iter().enumerate() {
            if (i >> index & 1_u64) == 1_u64 {
                this_set.push(content.clone());
            }
        }
        res.push(this_set);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::powerset;

    #[test]
    fn powerset_of_one_element_vector_yields_it_and_an_empty() {
        assert_eq!(powerset(&vec![1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn powerset_of_three_numbers_yields_eight_sets() {
        assert_eq!(powerset(&vec![1, 2, 3]),
                   vec![
            vec![],
            vec![1],
            vec![2],
            vec![1,2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1,2,3],
        ]);
    }

    #[test]
    fn powerset_of_4_numbers_has_16_elements() {
        assert_eq!(powerset(&vec![1, 2, 3, 4]).len(), 16);
    }
}
