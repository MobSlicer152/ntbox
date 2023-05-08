use exe::{headers, pe};
use log::{debug, error, info, trace, warn};
use std::{fs, io};

pub struct Module {
    pe: pe::VecPE,
    base: u64,
    sections: Vec<u64>
}

pub struct Loader {
    modules: Vec<Module>
}

impl Loader {
    pub fn new() -> Self {
        Self {
            modules: Vec::new()
        }
    }

    fn choose_addr(&self, emu: &unicorn_engine::Unicorn<()>, preferred: u64) -> u64 {
        0
    }

    pub fn load(&mut self, emu: &unicorn_engine::Unicorn<()>, path: &str) -> Result<&Module, exe::Error> {
        info!("Attempting to load PE binary {path}");
        let disk_pe: pe::VecPE = match pe::VecPE::from_disk_file(path) {
            Ok(pe) => pe,
            Err(err) => {
                error!("Failed to load PE binary {path}: {err:?}");
                return Err(err);
            }
        };
        let mem_pe_buf: Vec<u8> = match pe::PE::recreate_image(&disk_pe, pe::PEType::Memory) {
            Ok(pe) => pe,
            Err(err) => {
                error!("Failed to map PE binary {path}: {err:?}");
                return Err(err);
            }
        };

        let mem_pe = pe::VecPE::from_memory_data(&mem_pe_buf.into_boxed_slice());
        let nt_hdrs: &headers::ImageNTHeaders64 = match pe::PE::get_valid_nt_headers_64(&mem_pe) {
            Ok(hdrs) => hdrs,
            Err(err) => {
                error!("Failed to get NT image headers for {path}: {err:?}");
                return Err(err);
            }
        };
        let base = self.choose_addr(emu, nt_hdrs.optional_header.image_base);

        let module = Module {
            pe: mem_pe,
            base,
            sections: Vec::new()
        };
        self.modules.push(module);

        Ok(self.modules.last().unwrap())
    }
}
