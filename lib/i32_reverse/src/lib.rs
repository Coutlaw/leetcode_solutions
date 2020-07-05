/*
Given a 32-bit signed integer, reverse digits of an integer.
Example 1:
Input: 123
Output: 321
Example 2:
Input: -123
Output: -321
Example 3:
Input: 120
Output: 21
Note:
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
*/

// my solution
pub fn reverse(x: i32) -> i32 {
    // basic operation is to take the original number and cut off the tens digit
    // add that to a new variable until the original number is 0
    let mut reverse: i32 = 0;
    let mut over_flow_check: Option<i32> = None;
    let mut start_value = x;
    while start_value != 0 {
      // check for overflow with the new reversed number when we make room for a new digit
      over_flow_check = reverse.checked_mul(10);
      match over_flow_check {
        Some(x) => {
          reverse = x;
          // check that we can add the next value to the reversed number without causing overflow
          over_flow_check = reverse.checked_add(start_value % 10);
          match over_flow_check {
            Some(x) => reverse = x,
            None => return 0,
          }
        },
        None => return 0,
      }
      start_value = start_value / 10;
    }
    reverse
  }
  
  #[cfg(test)]
  mod test {
    use super::*;
  
    #[test]
    fn can_reverse() {
      let input1 = 123;
  
      let expected = 321;
      assert_eq!(expected, reverse(input1));
    }
  
    #[test]
    fn can_reverse2() {
      let input1 = 1534236469;
  
      let expected = 0;
      assert_eq!(expected, reverse(input1));
    }
  }