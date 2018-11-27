extern crate vswhom_sys;
extern crate libc;

use vswhom_sys::{vswhom_find_visual_studio_and_windows_sdk, vswhom_free_resources};
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
use libc::wcslen;
use std::slice;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VsFindResult {
    /// Zero if no Windows SDK found.
    pub windows_sdk_version: u8,

    pub windows_sdk_root: Option<OsString>,
    pub windows_sdk_um_library_path: Option<OsString>,
    pub windows_sdk_ucrt_library_path: Option<OsString>,

    pub vs_exe_path: Option<OsString>,
    pub vs_library_path: Option<OsString>,
}

impl VsFindResult {
    pub fn search() -> Option<VsFindResult> {
        let mut res = unsafe { vswhom_find_visual_studio_and_windows_sdk() };

        let ret = if res.windows_sdk_version != 0 {
            Some(VsFindResult {
                windows_sdk_version: res.windows_sdk_version as u8,

                windows_sdk_root: osfpo(res.windows_sdk_root),
                windows_sdk_um_library_path: osfpo(res.windows_sdk_um_library_path),
                windows_sdk_ucrt_library_path: osfpo(res.windows_sdk_ucrt_library_path),

                vs_exe_path: osfpo(res.vs_exe_path),
                vs_library_path: osfpo(res.vs_library_path),
            })
        } else {
            None
        };

        unsafe { vswhom_free_resources(&mut res) };

        ret
    }
}


fn osfpo(s: *const u16) -> Option<OsString> {
    if !s.is_null() {
        Some(OsString::from_wide(unsafe { slice::from_raw_parts(s, wcslen(s)) }))
    } else {
        None
    }
}


#[test]
fn tt() {
    println!("{:?}", VsFindResult::search());
    assert!(false);
}
