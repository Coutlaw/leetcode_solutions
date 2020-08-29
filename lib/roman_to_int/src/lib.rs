/*
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.
Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
Example 1:
Input: "III"
Output: 3
Example 2:
Input: "IV"
Output: 4
Example 3:
Input: "IX"
Output: 9
Example 4:
Input: "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.
Example 5:
Input: "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
*/

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
  let mut values = HashMap::new();

  values.insert('I', 1);
  values.insert('V', 5);
  values.insert('X', 10);
  values.insert('L', 50);
  values.insert('C', 100);
  values.insert('D', 500);
  values.insert('M', 1000);

  let mut total = 0;
  let mut previous = 0;

  for c in s.chars() {
    match values.get(&c) {
      Some(&value) => {
        if value > previous {
          if previous == 0 {
            total += value;
            previous = value;
            continue;
          }
          // add the difference of the value previously added and the new value
          total += value - (previous * 2);
          previous = value;
        } else if value <= previous {
          total += value;
          previous = value;
        }
      }
      _ => panic!("Invalid char"),
    }
  }
  total
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn can_convert_1() {
    let input = String::from("III");
    let expected = 3;
    assert_eq!(expected, roman_to_int(input));
  }

  #[test]
  fn can_convert_2() {
    let input = String::from("IV");
    let expected = 4;
    assert_eq!(expected, roman_to_int(input));
  }

  #[test]
  fn can_convert_3() {
    let input = String::from("IX");
    let expected = 9;
    assert_eq!(expected, roman_to_int(input));
  }

  #[test]
  fn can_convert_4() {
    let input = String::from("LVIII");
    let expected = 58;
    assert_eq!(expected, roman_to_int(input));
  }

  #[test]
  fn can_convert_5() {
    let input = String::from("MCMXCIV");
    let expected = 1994;
    assert_eq!(expected, roman_to_int(input));
  }
}