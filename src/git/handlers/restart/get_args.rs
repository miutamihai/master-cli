use crate::common::str_vec_to_string_vec::str_vec_to_string_vec;

fn build_static(args: Vec<&str>, dynamic: String) -> Vec<String> {
    [str_vec_to_string_vec(args), vec![dynamic]].concat()
}

pub fn get_args(destination: &String, origin: &String) -> Vec<Vec<String>> {
    vec![
        build_static(vec!["checkout"], origin.clone()),
        build_static(vec!["pull", "origin"], origin.clone()),
        build_static(vec!["branch", "-D"], destination.clone()),
        build_static(vec!["checkout", "-b"], destination.clone()),
    ]
}
