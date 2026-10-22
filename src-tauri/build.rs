fn main() {
    #[cfg(windows)]
    {
        // 集成测试会链接 rfd 的 TaskDialogIndirect。测试 exe 默认没有 Common Controls v6
        // manifest，Windows 会加载旧 comctl32.dll 并在进程启动前报 ENTRYPOINT_NOT_FOUND。
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("windows-test.manifest");
        println!("cargo:rerun-if-changed=windows-test.manifest");
        println!("cargo:rustc-link-arg-tests=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-tests=/MANIFESTINPUT:{}", manifest.display());
    }
    tauri_build::build()
}
