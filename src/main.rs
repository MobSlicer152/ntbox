use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::{info};
use std::{fs, io};
use unicorn_engine::unicorn_const;

mod loader;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub struct DataDirs;
impl DataDirs {
    pub fn all() -> Vec<String> {
        vec![Self::base(), Self::logs()]
    }

    pub fn base() -> String {
        let basedirs = directories::BaseDirs::new().unwrap();
        let subdir_path = basedirs.data_dir().to_str().unwrap().replace('\\', "/");
        format!("{subdir_path}/ntbox/")
    }

    pub fn logs() -> String {
        Self::base() + "logs/"
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let dt = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::BrightCyan)
        .trace(Color::Cyan);

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            let dt = Local::now();
            out.finish(format_args!(
                "\x1B[{}m[{} {} {}] {}\x1B[0m",
                colors_line.get_color(&record.level()).to_fg_str(),
                dt.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file(
            DataDirs::logs() + "ntbox-" + &dt + ".log",
        )?);

    #[cfg(build = "debug")]
    let dispatch = dispatch.level(log::LevelFilter::Debug);
    #[cfg(all(not(build = "debug"), feature = "release_log"))]
    let dispatch = dispatch.level(log::LevelFilter::Info);
    #[cfg(any(build = "debug", all(not(build = "debug"), feature = "release_log")))]
    let dispatch = dispatch.chain(io::stdout());

    dispatch.apply()?;

    Ok(())
}

fn main() {
    for dir in DataDirs::all() {
        if fs::create_dir_all(dir.clone()).is_err() {
            panic!("Failed to create data directory {dir}")
        }
    }

    if setup_logger().is_err() {
        panic!("Failed to set up logger")
    }

    info!("Initializing ntbox");
    
    info!("Initializing emulation");
    let mut emu = match unicorn_engine::Unicorn::new(unicorn_const::Arch::X86, unicorn_const::Mode::MODE_64) {
        Ok(emu) => emu,
        Err(err) => panic!("Failed to create Unicorn instance: {err:?}")
    };
    
}
