extern crate vswhom_sys;
#[cfg(target_os = "windows")]
extern crate libc;

#[cfg(target_os = "windows")]
use vswhom_sys::{vswhom_find_visual_studio_and_windows_sdk, vswhom_free_resources};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use vswhom_sys::Find_Result;
use std::num::NonZeroU8;
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use libc::wcslen;
#[cfg(target_os = "windows")]
use std::slice;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VsFindResult {
    /// Zero if no Windows SDK found.
    pub windows_sdk_version: NonZeroU8,

    pub windows_sdk_root: Option<OsString>,
    pub windows_sdk_um_library_path: Option<OsString>,
    pub windows_sdk_ucrt_library_path: Option<OsString>,

    pub vs_exe_path: Option<OsString>,
    pub vs_library_path: Option<OsString>,
}

impl VsFindResult {
    pub fn search() -> Option<VsFindResult> {
        #[cfg(target_os = "windows")]
        unsafe {
            let mut res = vswhom_find_visual_studio_and_windows_sdk();
            let ret = VsFindResult::from_raw_result(&res);
            vswhom_free_resources(&mut res);
            ret
        }

        #[cfg(not(target_os = "windows"))]
        None
    }

    pub unsafe fn from_raw_result(res: &Find_Result) -> Option<VsFindResult> {
        #[cfg(target_os = "windows")]
        {
            if res.windows_sdk_version != 0 {
                Some(VsFindResult {
                    windows_sdk_version: NonZeroU8::new_unchecked(res.windows_sdk_version as u8),

                    windows_sdk_root: osfpo(res.windows_sdk_root),
                    windows_sdk_um_library_path: osfpo(res.windows_sdk_um_library_path),
                    windows_sdk_ucrt_library_path: osfpo(res.windows_sdk_ucrt_library_path),

                    vs_exe_path: osfpo(res.vs_exe_path),
                    vs_library_path: osfpo(res.vs_library_path),
                })
            } else {
                None
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = res;
            None
        }
    }
}


#[cfg(target_os = "windows")]
unsafe fn osfpo(s: *const u16) -> Option<OsString> {
    if !s.is_null() {
        Some(OsString::from_wide(slice::from_raw_parts(s, wcslen(s))))
    } else {
        None
    }
}
