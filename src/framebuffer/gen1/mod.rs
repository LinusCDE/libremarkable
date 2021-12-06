pub mod base;
pub mod io;
pub mod refresh;

use memmap2::MmapRaw;

use std::fs::File;
use std::sync::atomic::AtomicU32;

use crate::framebuffer::screeninfo::{FixScreeninfo, VarScreeninfo};

/// Framebuffer struct containing the state (latest update marker etc.)
/// along with the var/fix screeninfo structs.
pub struct Gen1 {
    pub device: File,
    pub frame: MmapRaw,
    pub marker: AtomicU32,
    /// Not updated as a result of calling `Framebuffer::put_var_screeninfo(..)`.
    /// It is your responsibility to update this when you call into that function
    /// like it has been done in `Framebuffer::new(..)`.
    pub var_screen_info: VarScreeninfo,
    pub fix_screen_info: FixScreeninfo,
}

unsafe impl Send for Gen1 {}
unsafe impl Sync for Gen1 {}
