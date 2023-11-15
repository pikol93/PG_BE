use flexi_logger::{
    style, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming, Record,
};
use log::{error, info};
use std::{panic::PanicInfo, thread};

const TIME_FORMAT_SHORT: &str = "%H:%M:%S%.3f";
const TIME_FORMAT_LONG: &str = "%Y-%m-%d %H:%M:%S%.6f";

pub fn initialize_logger() {
    std::panic::set_hook(Box::new(on_panic));

    let filespec = FileSpec::default()
        .basename("log")
        .directory("logs/");

    let logger = Logger::try_with_env_or_str(
        "trace, bevy=off, wgpu=warn, wgpu=warn, mio=warn, winit=warn, gilrs=warn, naga=warn",
    )
    .expect("Should succeed initializing with env.")
    .format_for_files(format_with_thread)
    .format_for_stdout(format_colored_with_thread)
    .log_to_file(filespec)
    .rotate(
        Criterion::Size(10 * 1024 * 1024),
        Naming::Timestamps,
        Cleanup::KeepLogFiles(5),
    )
    .duplicate_to_stdout(Duplicate::All);

    match logger.start() {
        Ok(_) => info!("Logging initialized."),
        Err(_) => panic!("Could not initialize logging."),
    };
}

fn on_panic(panic_info: &PanicInfo) {
    error!("Application panicked. {}", panic_info);
}

fn format_colored_with_thread(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "[{}] [{}] [{}] [{}:{}] {}",
        style(level).paint(now.format(TIME_FORMAT_SHORT).to_string()),
        style(level).paint(format!("{:<5}", level.to_string())),
        style(level).paint(thread::current().name().unwrap_or("<unnamed>")),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        style(level).paint(&record.args().to_string())
    )
}

fn format_with_thread(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "[{}] | [{:<5}] | [{}] | [{}:{}] | {}",
        now.format(TIME_FORMAT_LONG).to_string(),
        level.to_string(),
        thread::current().name().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args().to_string()
    )
}
