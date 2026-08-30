fn sort<T: PartialOrd>(values: Vec<T>) -> Vec<T> {
    // insertion sort

    let mut result = Vec::with_capacity(values.len());
    for v in values {
        let mut current_index = result.len();

        while current_index > 0 && v < result[current_index - 1] {
            current_index -= 1;
        }
        result.insert(current_index, v);
    }

    result
}

fn main() {
    let input_args = std::env::args().skip(1);

    // let mut args = Vec::from_iter(input_args);
    let mut args = Vec::with_capacity(input_args.len());
    for arg in input_args {
        args.push(arg);
    }

    // option 1
    // args.sort();
    // option 1 (b) - case insensitive
    // args.sort_by_key(|x| x.to_lowercase());

    // option 2
    args = sort(args);

    for arg in args {
        println!("{}", arg);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let vector: Vec<i32> = vec![];
        assert_eq!(sort(vector), vec![]);
    }

    #[test]
    fn sort_positive_integers() {
        assert_eq!(
            sort(vec![10, 5, 0]),
            vec![0, 5, 10]
        );
    }

    #[test]
    fn sort_equal_numbers() {
        assert_eq!(
            sort(vec![1, 1, 1, 1, 1]),
            vec![1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn sort_pos_and_neg_integers() {
        assert_eq!(
            sort(vec![-5, 10, -15, 25, -50]),
            vec![-50, -15, -5, 10, 25]
        );
    }

    #[test]
    fn sort_strings() {
        assert_eq!(
            sort(vec!["c", "b", "a"]),
            vec!["a", "b", "c"]
        )
    }
}