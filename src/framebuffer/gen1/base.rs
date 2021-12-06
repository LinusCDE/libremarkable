use libc::ioctl;
use memmap2::MmapOptions;

use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::sync::atomic::AtomicU32;

use crate::framebuffer;
use crate::framebuffer::common::{
    FBIOGET_FSCREENINFO, FBIOGET_VSCREENINFO, FBIOPUT_VSCREENINFO, MXCFB_DISABLE_EPDC_ACCESS,
    MXCFB_ENABLE_EPDC_ACCESS, MXCFB_SET_AUTO_UPDATE_MODE, MXCFB_SET_UPDATE_SCHEME,
};
use crate::framebuffer::gen1::Gen1;
use crate::framebuffer::screeninfo::{FixScreeninfo, VarScreeninfo};

impl framebuffer::FramebufferBase for Gen1 {
    fn from_path(path_to_device: &str) -> Gen1 {
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path_to_device)
            .unwrap();

        let mut var_screen_info = Gen1::get_var_screeninfo(&device);
        var_screen_info.xres = 1404;
        var_screen_info.yres = 1872;
        var_screen_info.rotate = 1;
        var_screen_info.width = 0xffff_ffff;
        var_screen_info.height = 0xffff_ffff;
        var_screen_info.pixclock = 6250;
        var_screen_info.left_margin = 32;
        var_screen_info.right_margin = 326;
        var_screen_info.upper_margin = 4;
        var_screen_info.lower_margin = 12;
        var_screen_info.hsync_len = 44;
        var_screen_info.vsync_len = 1;
        var_screen_info.sync = 0;
        var_screen_info.vmode = 0; // FB_VMODE_NONINTERLACED
        var_screen_info.accel_flags = 0;

        Gen1::put_var_screeninfo(&device, &mut var_screen_info);

        let fix_screen_info = Gen1::get_fix_screeninfo(&device);
        let frame_length = (fix_screen_info.line_length * var_screen_info.yres) as usize;

        let mem_map = MmapOptions::new()
            .len(frame_length)
            .map_raw(&device)
            .expect("Unable to map provided path");

        Gen1 {
            marker: AtomicU32::new(1),
            device,
            frame: mem_map,
            var_screen_info,
            fix_screen_info,
        }
    }

    fn set_epdc_access(&mut self, state: bool) {
        unsafe {
            libc::ioctl(
                self.device.as_raw_fd(),
                if state {
                    MXCFB_ENABLE_EPDC_ACCESS
                } else {
                    MXCFB_DISABLE_EPDC_ACCESS
                },
            );
        };
    }

    fn set_autoupdate_mode(&mut self, mode: u32) {
        let m = mode.to_owned();
        unsafe {
            libc::ioctl(
                self.device.as_raw_fd(),
                MXCFB_SET_AUTO_UPDATE_MODE,
                &m as *const u32,
            );
        };
    }

    fn set_update_scheme(&mut self, scheme: u32) {
        let s = scheme.to_owned();
        unsafe {
            libc::ioctl(
                self.device.as_raw_fd(),
                MXCFB_SET_UPDATE_SCHEME,
                &s as *const u32,
            );
        };
    }

    fn get_fix_screeninfo(device: &File) -> FixScreeninfo {
        let mut info: FixScreeninfo = Default::default();
        let result = unsafe { ioctl(device.as_raw_fd(), FBIOGET_FSCREENINFO, &mut info) };
        assert!(result == 0, "FBIOGET_FSCREENINFO failed");
        info
    }

    fn get_var_screeninfo(device: &File) -> VarScreeninfo {
        let mut info: VarScreeninfo = Default::default();
        let result = unsafe { ioctl(device.as_raw_fd(), FBIOGET_VSCREENINFO, &mut info) };
        assert!(result == 0, "FBIOGET_VSCREENINFO failed");
        info
    }

    fn put_var_screeninfo(device: &std::fs::File, var_screen_info: &mut VarScreeninfo) -> bool {
        let result = unsafe { ioctl(device.as_raw_fd(), FBIOPUT_VSCREENINFO, var_screen_info) };
        result == 0
    }

    fn update_var_screeninfo(&mut self) -> bool {
        Self::put_var_screeninfo(&self.device, &mut self.var_screen_info)
    }
}
