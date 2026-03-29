use anyhow::Result;
use chrono::Local;
use fern::Dispatch;
use std::cell::Cell;

thread_local! {
    /// Флаг тихого режима: если true, game_info! не пишет в лог.
    /// Используется при прогонке тестовых ходов (симуляция).
    static SILENT: Cell<bool> = const { Cell::new(false) };
}

/// Включить/выключить тихий режим логирования
pub fn set_silent(silent: bool) {
    SILENT.with(|s| s.set(silent));
}

/// Проверить, включён ли тихий режим
pub fn is_silent() -> bool {
    SILENT.with(|s| s.get())
}

/// Макрос для игрового логирования: пишет info! только если не в тихом режиме.
#[macro_export]
macro_rules! game_info {
    ($($arg:tt)*) => {
        if !$crate::logger::is_silent() {
            log::info!($($arg)*);
        }
    };
}

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
