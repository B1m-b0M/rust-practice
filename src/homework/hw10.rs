fn is_palindrome(mut x: u32) -> bool {
    let original = x;
    let mut reversed = 0;

    while x > 0 {
        let digit = x % 10;
        reversed = reversed * 10 + digit;
        x /= 10;
    }

    original == reversed
}


 #[test]
fn test_is_palindrome() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];
    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
 }
