// 跨平台文件与系统路径辅助。业务代码只依赖这里的抽象，避免把 /tmp 或
// Unix 的 rename 语义带进 Windows。

use std::path::{Path, PathBuf};

#[cfg(not(target_os = "windows"))]
use std::fs;

// 默认运行日志目录：macOS 继续沿用旧位置，其他桌面平台放到系统临时目录，
// 避免依赖盘符根目录下的 /tmp。
pub fn default_log_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        PathBuf::from("/tmp/ccproxy")
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::env::temp_dir().join("ccproxy")
    }
}

// 用同目录临时文件替换目标文件。Unix 的 rename 会覆盖，Windows 则必须显式请求
// 替换已有文件；两种平台都只在替换这一步失败时返回错误。
pub fn replace_file(temp_path: &Path, target_path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        replace_file_windows(temp_path, target_path)
    }
    #[cfg(not(target_os = "windows"))]
    {
        fs::rename(temp_path, target_path).map_err(|error| error.to_string())
    }
}

#[cfg(target_os = "windows")]
fn replace_file_windows(temp_path: &Path, target_path: &Path) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH};

    let temp: Vec<u16> = temp_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let target: Vec<u16> = target_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let ok = unsafe {
        MoveFileExW(
            temp.as_ptr(),
            target.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if ok == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(())
}
