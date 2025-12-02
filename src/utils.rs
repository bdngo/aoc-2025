use std::str::FromStr;

pub fn string_to_ints<T>(s: String) -> Vec<T>
where
    T: FromStr,
{
    s.split_whitespace()
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}
