use exe::pe;
use log::{debug, error, info, trace, warn};
use std::{fs, io};

pub struct Loader {
    modules: Vec<pe::VecPE>
}

impl Loader {
    pub fn new() -> Self {
        Self {
            modules: Vec::new()
        }
    }

    pub fn load(&mut self, path: &str) -> Result<&pe::VecPE, exe::Error> {
        info!("Attempting to load PE binary {path}");
        let disk_pe: pe::VecPE = match pe::VecPE::from_disk_file(path) {
            Ok(pe) => pe,
            Err(err) => {
                error!("Failed to load PE binary {path}: {err:?}");
                return Err(err);
            }
        };
        let mem_pe: Vec<u8> = match pe::PE::recreate_image(&disk_pe, pe::PEType::Memory) {
            Ok(pe) => pe,
            Err(err) => {
                error!("Failed to map PE binary {path}: {err:?}");
                return Err(err);
            }
        };
        self.modules.push(pe::VecPE::from_memory_data(&mem_pe.into_boxed_slice()));
        Ok(self.modules.last().unwrap())
    }
}
