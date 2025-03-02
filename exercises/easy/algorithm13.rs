/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::{collections::HashMap, fmt::{self, Display, Formatter}, ops::Index};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    let mut ret = true;
    let mut hashmap_use = HashMap::<char, usize>::new();
    for c in s1.chars() {
        if c.is_alphabetic() {
            let cur_c = c.to_ascii_lowercase();
            if !hashmap_use.contains_key(&cur_c) {
                hashmap_use.insert(cur_c, 0);
            }
            let cnt = hashmap_use.get_mut(&cur_c).unwrap();
            *cnt += 1;
        }
    }
    for c in s2.chars() {
        if c.is_alphabetic() {
            let cur_c = c.to_ascii_lowercase();
            if hashmap_use.contains_key(&cur_c) {
                let cnt = hashmap_use.get_mut(&cur_c).unwrap();
                if *cnt == 1 {
                    hashmap_use.remove(&cur_c);
                } else {
                    *cnt -= 1;
                }
            } else {
                ret = false;
                break;
            }
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}