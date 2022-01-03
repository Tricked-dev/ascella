use std::env;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    home_dir_inner()
}

#[cfg(any(unix, target_os = "redox"))]
fn home_dir_inner() -> Option<PathBuf> {
    #[allow(deprecated)]
    env::home_dir()
}

#[cfg(windows)]
pub fn home_dir_inner() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(home_dir_crt)
}

#[cfg(windows)]
#[cfg(not(target_vendor = "uwp"))]
fn home_dir_crt() -> Option<PathBuf> {
    unsafe {
        let mut path: Vec<u16> = Vec::with_capacity(MAX_PATH);
        match SHGetFolderPathW(
            ptr::null_mut(),
            CSIDL_PROFILE,
            ptr::null_mut(),
            0,
            path.as_mut_ptr(),
        ) {
            S_OK => {
                let len = wcslen(path.as_ptr());
                path.set_len(len);
                let s = OsString::from_wide(&path);
                Some(PathBuf::from(s))
            }
            _ => None,
        }
    }
}

#[cfg(windows)]
#[cfg(target_vendor = "uwp")]
fn home_dir_crt() -> Option<PathBuf> {
    None
}
