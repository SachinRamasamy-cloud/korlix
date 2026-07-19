use std::path::Path;

pub fn copy_public_assets(public_dir: &Path, dist_dir: &Path) -> std::io::Result<()> {
    if !public_dir.exists() {
        return Ok(());
    }
    copy_dir_recursive(public_dir, dist_dir)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
