use rustc_hash::FxHashMap;
use std::{char, collections::HashMap};

pub fn is_anagram_sort(word1: impl Into<String>, word2: impl Into<String>) -> bool {
    let mut word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let mut word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    if word1.len() != word2.len() {
        return false;
    }

    word1.sort();
    word2.sort();

    word1 == word2
}

pub fn is_anagram_hashmap(word1: impl Into<String>, word2: impl Into<String>) -> bool {
    let word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    if word1.len() != word2.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    for i in 0..word1.len() {
        *map.entry(word1[i]).or_default() += 1;
        *map.entry(word2[i]).or_default() -= 1;
    }

    map.values().all(|v| *v == 0)
}

pub fn is_anagram_faster_hash(word1: impl Into<String>, word2: impl Into<String>) -> bool {
    let word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    if word1.len() != word2.len() {
        return false;
    }

    let mut map: FxHashMap<char, i32> = FxHashMap::default();

    for i in 0..word1.len() {
        *map.entry(word1[i]).or_default() += 1;
        *map.entry(word2[i]).or_default() -= 1;
    }

    map.values().all(|v| *v == 0)
}

pub fn is_anagram_vector(word1: impl Into<String>, word2: impl Into<String>) -> bool {
    let word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    if word1.len() != word2.len() {
        return false;
    }
    let mut counts: Vec<i16> = vec![0; char::MAX as usize];
    for i in 0..word1.len() {
        counts[word1[i] as usize] += 1;
        counts[word2[i] as usize] -= 1;
    }

    counts.iter().all(|v| *v == 0)
}

pub fn is_anagram_ascii_array(
    word1: impl Into<String>,
    word2: impl Into<String>,
) -> Result<bool, &'static str> {
    let word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    const ASCII_SIZE: usize = 256;
    if word1.len() != word2.len() {
        return Ok(false);
    }
    let err_message = "invalid character";
    let mut counts: [i16; ASCII_SIZE] = [0; ASCII_SIZE];
    for i in 0..word1.len() {
        let index1 = word1[i] as usize;
        let index2 = word2[i] as usize;
        if index1 > ASCII_SIZE || index2 > ASCII_SIZE {
            return Err(err_message);
        }
        counts[index1] += 1;
        counts[index2] -= 1;
    }

    Ok(counts.iter().all(|v| *v == 0))
}

pub fn is_anagram_ascii_array_panics(word1: impl Into<String>, word2: impl Into<String>) -> bool {
    let word1: Vec<char> = Into::<String>::into(word1).chars().collect();
    let word2: Vec<char> = Into::<String>::into(word2).chars().collect();
    const ASCII_SIZE: usize = 256;
    if word1.len() != word2.len() {
        return false;
    }
    let mut counts: [i16; ASCII_SIZE] = [0; ASCII_SIZE];
    for i in 0..word1.len() {
        counts[word1[i] as usize] += 1;
        counts[word2[i] as usize] -= 1;
    }

    counts.iter().all(|v| *v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_sort_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, bool)> = vec![
            ("abc", "cba", true),
            ("abc", "caa", false),
            ("abc", "acba", false),
            ("roma", "amor", true),
            ("leadership", "dealership", true),
            ("leadership", "dealershii", false),
            ("横山ダニエル", "ダニエル横山", true),
            ("横山ダニエル", "横山daniel", false),
            ("横山ダニエル", "横山daniel......", false),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_sort(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    fn is_anagram_hashmap_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, bool)> = vec![
            ("abc", "cba", true),
            ("abc", "caa", false),
            ("abc", "acba", false),
            ("roma", "amor", true),
            ("leadership", "dealership", true),
            ("leadership", "dealershii", false),
            ("横山ダニエル", "ダニエル横山", true),
            ("横山ダニエル", "横山daniel", false),
            ("横山ダニエル", "横山daniel......", false),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_hashmap(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    fn is_anagram_faster_hash_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, bool)> = vec![
            ("abc", "cba", true),
            ("abc", "caa", false),
            ("abc", "acba", false),
            ("roma", "amor", true),
            ("leadership", "dealership", true),
            ("leadership", "dealershii", false),
            ("横山ダニエル", "ダニエル横山", true),
            ("横山ダニエル", "横山daniel", false),
            ("横山ダニエル", "横山daniel......", false),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_faster_hash(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    fn is_anagram_vector_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, bool)> = vec![
            ("abc", "cba", true),
            ("abc", "caa", false),
            ("abc", "acba", false),
            ("roma", "amor", true),
            ("leadership", "dealership", true),
            ("leadership", "dealershii", false),
            ("横山ダニエル", "ダニエル横山", true),
            ("横山ダニエル", "横山daniel", false),
            ("横山ダニエル", "横山daniel......", false),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_vector(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    fn is_anagram_ascii_array_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, Result<bool, &'static str>)> = vec![
            ("abc", "cba", Ok(true)),
            ("abc", "caa", Ok(false)),
            ("abc", "acba", Ok(false)),
            ("roma", "amor", Ok(true)),
            ("leadership", "dealership", Ok(true)),
            ("leadership", "dealershii", Ok(false)),
            ("横山ダニエル", "ダニエル横山", Err("invalid character")),
            ("横山ダニエル", "横山daniel", Ok(false)),
            ("横山ダニエル", "横山dani", Err("invalid character")),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_ascii_array(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    fn is_anagram_ascii_array_panics_tests() {
        // arrange
        let test_table: Vec<(&'static str, &'static str, bool)> = vec![
            ("abc", "cba", true),
            ("abc", "caa", false),
            ("abc", "acba", false),
            ("roma", "amor", true),
            ("leadership", "dealership", true),
            ("leadership", "dealershii", false),
        ];

        for (word1, word2, expected) in test_table {
            // act
            let res = is_anagram_ascii_array_panics(word1, word2);

            // assert
            assert_eq!(res, expected, "Test {}:{}", word1, word2);
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn is_anagram_ascii_array_panics_invalid_tests() {
        // act
        _ = is_anagram_ascii_array_panics("横山ダニエル", "横山ダニエル");
    }
}
