pub fn is_leap_year(year: u64) -> bool {
    let reminder_4 = year % 4;
    let reminder_100 = year % 100;
    let reminder_400 = year % 400;

    match (reminder_4, reminder_100, reminder_400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        _ => false,
    }
}
