pub fn str_vec_to_string_vec(vec: Vec<&str>) -> Vec<String> {
    vec.iter()
        .map(|value| String::from(*value))
        .collect()
}
