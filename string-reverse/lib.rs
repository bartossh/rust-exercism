pub fn reverse(input: &str) -> String {
    let reversed_str = input.chars().rev().collect::<String>();
    reversed_str
}
