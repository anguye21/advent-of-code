pub fn str_to_vec(s: impl Into<String>, split: &str) -> Vec<String> {
    let val = s.into();

    return val
        .split(split)
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
}
