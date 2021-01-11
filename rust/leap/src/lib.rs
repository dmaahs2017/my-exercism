pub fn is_leap_year(year: u64) -> bool {
    // truth table for where it should be true
    // by4 | by100 | by400 |   result
    // yes   no      *         yes
    // yes   yes     yes       yes
    (year % 4 == 0) && ((year % 100 != 0) || (year % 100 == 0 && year % 400 == 0))
}
