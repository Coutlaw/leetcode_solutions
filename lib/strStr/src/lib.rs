/*
Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "hello", needle = "ll"
Output: 2

Example 2:

Input: haystack = "aaaaa", needle = "bba"
Output: -1

Clarification:

What should we return when needle is an empty string? This is a great question to ask during an interview.

For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().



Constraints:

    haystack and needle consist only of lowercase English characters.


*/

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 || haystack[..] == needle[..] {
        return 0;
    }
    if needle.len() > haystack.len() {
        return -1;
    }
    let needle_len = needle.len();
    for i in 0..haystack.len() - needle_len + 1 {
        if needle == haystack[i..i + needle_len] {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = String::from("hello");
        let needle = String::from("ba");
        let answer = -1;
        assert_eq!(str_str(haystack, needle), answer);
    }

    #[test]
    fn it_works2() {
        let haystack = String::from("hello");
        let needle = String::from("ll");
        let answer = 2;
        assert_eq!(str_str(haystack, needle), answer);
    }
}
