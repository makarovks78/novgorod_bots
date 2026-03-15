use anyhow::Result;
use chrono::Local;
use fern::Dispatch;

pub fn init_logger() -> Result<()> {
    // Имя лог-файла с меткой времени
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let file_name = format!("logs/game_log_{}.log", timestamp);

    // Настройка логгера
    Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("{}", message)))
        .level(log::LevelFilter::Debug) // минимальный уровень логирования
        .chain(std::io::stdout()) // также в stdout
        .chain(fern::log_file(file_name)?) // и в файл
        .apply()?;

    Ok(())
}
