use std::collections::HashMap;

pub fn reduce(_word: &str, counts: Vec<i32>) -> i32 {
    counts.iter().sum()
}


pub fn shuffle_and_group(map_output:   Vec<(String, i32)>) -> HashMap<String, Vec<i32>>{
    let mut grouped_data = HashMap::new();

    for (word, count) in map_output {
        grouped_data.entry(word).or_insert(Vec::new()).push(count);
    }
    grouped_data

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