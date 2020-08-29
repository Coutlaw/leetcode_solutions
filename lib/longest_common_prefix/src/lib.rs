/*
Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".
Example 1:
Input: ["flower","flow","flight"]
Output: "fl"
Example 2:
Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
Note:
All given inputs are in lowercase letters a-z.
words
  .map(_.chars())
  .fold(_.zip(_).take_while(==))
*/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    use std::str::Chars;
    let mut prefix = String::new();
    let mut iters: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
    let mut curr_char: Option<char> = None;
    if strs.len() < 1 {
      return prefix;
    }
    loop {
      curr_char.take().map(|ch| prefix.push(ch));
      for iter in iters.iter_mut() {
        let mut ch = iter.next();
        if ch.is_none() {
          return prefix;
        }
        match curr_char {
          None => curr_char = ch.take(),
          Some(curr) => {
            if curr != ch.unwrap() {
              return prefix;
            }
          }
        }
      }
    }
  }
  
  #[cfg(test)]
  mod test {
    use super::*;
  
    #[test]
    fn can_convert_1() {
      let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
      ];
      let expected = String::from("fl");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_2() {
      let input = vec![
        String::from("dog"),
        String::from("car"),
        String::from("racecar"),
      ];
      let expected = String::from("");
      assert_eq!(expected, longest_common_prefix(input));
    }
    #[test]
    fn can_convert_3() {
      let input = vec![String::from("a")];
      let expected = String::from("a");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_4() {
      let input = vec![];
      let expected = String::new();
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_5() {
      let input = vec![String::from("a"), String::from("a")];
      let expected = String::from("a");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_6() {
      let input = vec![String::from("ca"), String::from("a")];
      let expected = String::from("");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_7() {
      let input = vec![String::from("aa"), String::from("a")];
      let expected = String::from("a");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_8() {
      let input = vec![String::from("abab"), String::from("aba"), String::new()];
      let expected = String::from("");
      assert_eq!(expected, longest_common_prefix(input));
    }
  
    #[test]
    fn can_convert_9() {
      let input = vec![
        String::from("ac"),
        String::from("ac"),
        String::from("a"),
        String::from("a"),
      ];
      let expected = String::from("a");
      assert_eq!(expected, longest_common_prefix(input));
    }
  }