fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    
    let effective_shift = ((n % len) + len) % len;
    let split_point = (len - effective_shift) as usize;

    
    format!("{}{}", &s[split_point..], &s[..split_point])
}

#[test]
fn test_rotate() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    for &(n, expected) in shifts.iter() {
        assert_eq!(rotate(s.clone(), n), expected.to_string());
    }
}
