/*
Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
Example 1:
Input: 121
Output: true
Example 2:
Input: -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:
Input: 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
Follow up:
Could you solve it without converting the integer to a string?
*/


// my solution
pub fn is_palindrome(x: i32) -> bool {
    let mut int_as_string = String::from(format!("{}", x));
  
    while int_as_string.len() > 1 {
      let head = int_as_string.remove(0);
      let tail = int_as_string.pop().unwrap();
      if head != tail { return false; }
    }
    true
  }
  
  #[cfg(test)]
  mod test {
    use super::*;
  
    #[test]
    fn can_find_palindrome1() {
      let input1 = 121;
  
      let expected = true;
      assert_eq!(expected, is_palindrome(input1));
    }
  
    #[test]
    fn can_find_palindrome2() {
      let input1 = 2222;
  
      let expected = true;
      assert_eq!(expected, is_palindrome(input1));
    }
  
    #[test]
    fn can_find_palindrome3() {
      let input1 = 0;
  
      let expected = true;
      assert_eq!(expected, is_palindrome(input1));
    }
  
    #[test]
    fn can_find_palindrome4() {
      let input1 = 1234;
  
      let expected = false;
      assert_eq!(expected, is_palindrome(input1));
    }
  
  }