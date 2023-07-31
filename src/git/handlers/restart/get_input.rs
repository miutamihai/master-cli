use log::info;

pub fn get_input(message: String) -> String {
    info!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
