
pub fn reduce(_word: &str, counts: Vec<i32>) -> i32 {
    counts.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_reduce_function() {
        let word = "hello";
        let counts = vec![1, 1, 1, 1]; 
        let expected_sum = 4;

        let result = reduce(word, counts);

        assert_eq!(result, expected_sum);
    }
}