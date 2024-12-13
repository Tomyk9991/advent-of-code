pub fn greatest_common_divisor(number1: i64, number2: i64, target_number: i64) -> Option<(i64, i64)> {
    let (gcd, x, y) = extended_gcd(number1, number2);

    if target_number % gcd != 0 {
        return None; // No solution exists
    }

    let scale = target_number / gcd;
    Some((x * scale, y * scale))
}

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}