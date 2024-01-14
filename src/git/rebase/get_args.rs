use crate::common::traits::as_string_vec::AsStringVec;

fn build_static(args: Vec<&'static str>, dynamic: String) -> Vec<String> {
    [args.as_string_vec(), vec![dynamic]].concat()
}

pub fn get_args(destination: &String, origin: &String) -> Vec<Vec<String>> {
    vec![
        build_static(vec!["checkout"], origin.clone()),
        build_static(vec!["pull", "origin"], origin.clone()),
        build_static(vec!["checkout"], destination.clone()),
        build_static(vec!["rebase", "-i"], origin.clone()),
    ]
}
