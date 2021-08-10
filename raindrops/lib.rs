pub fn raindrops(n: u32) -> String {
    let factor_7 = n % 7 == 0;
    let factor_5 = n % 5 == 0;
    let factor_3 = n % 3 == 0;
    if factor_3 || factor_5 || factor_7 {
        let mut result  = "".to_owned();
        if factor_3 {
            result.push_str("Pling");
        }
        if factor_5 {
            result.push_str("Plang");
        }
        if factor_7 {
            result.push_str("Plong");
        }
        return result.to_owned();
    }
    n.to_string().to_owned()
}
