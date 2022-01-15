pub struct Writer(pub(crate) *mut sys::Ptex_PtexWriter_t);

use crate::{core,sys};

impl Drop for Writer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                sys::Ptex_PtexWriter_release(self.0);
            }
            self.0 = std::ptr::null_mut();
        }
    }
}

impl Writer {
    pub fn new(filename: &std::path::PathBuf) -> Self {
        Writer(std::ptr::null_mut())
    }
}

