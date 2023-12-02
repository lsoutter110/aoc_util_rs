pub fn atoi(a: &str, radix: u32) -> u32 {
    if radix > 16 {
        panic!("Error while parsing base {radix} number: base too large, max is base 16!");
    }

    return a.chars().fold(0, |i, c| i*radix + match c.to_digit(radix) {
        Some(n) => n,
        None => panic!("Error while parsing base {radix} number '{a}': unexpected character '{c}'!"),
    });
}