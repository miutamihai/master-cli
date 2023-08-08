use fern::colors::{Color, ColoredLevelConfig};
use log::SetLoggerError;

fn get_logger(colors: ColoredLevelConfig) -> Result<(), SetLoggerError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
}

fn get_color_config() -> ColoredLevelConfig {
    ColoredLevelConfig::new().info(Color::Cyan)
}

pub fn setup_logger() {
    let colors = get_color_config();

    match get_logger(colors) {
        Ok(_) => {}
        Err(_) => {}
    }
}
