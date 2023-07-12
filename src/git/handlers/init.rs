use std::env;

fn is_work_dir() -> bool {
    let work_dir = "sevencode";

    match env::current_dir() {
        Ok(current_dir) => {
            current_dir.components()
                .any(|value| value.as_os_str().eq(work_dir))
        }
        Err(_) => { false }
    }
}

pub fn init() -> Option<()> {
    println!("Running git init");

    println!("is work dir {}", is_work_dir());

    None
}
