pub(crate) mod emu {
    use log::{error, info};
    use unicorn_engine::unicorn_const;

    pub fn init() -> unicorn_engine::Unicorn {
        info!("Initializing emulation");

        let mut emu = unicorn_engine::Unicorn::new(unicorn_engine::Arch::X86, unicorn_engine::Mode::MODE_64);
        emu
    }

    pub fn shutdown() {
    }
}
