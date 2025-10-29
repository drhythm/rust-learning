//https://leetcode.com/problems/smallest-number-with-all-set-bits
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut a = 0;
        for number in (1..31) {
            a = 1 << number;
            if n < a {
                break;
            }
        }
        a - 1
    }
}
