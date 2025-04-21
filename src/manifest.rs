use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicU16, Ordering};
use camino::Utf8Path;

const MANIFEST_RESOURCE_SCRIPT: &str = "#define RT_MANIFEST 24
[ID] RT_MANIFEST \"[PATH]\"\n";
pub(crate) static CURRENT_MANIFEST_ID: AtomicU16 = AtomicU16::new(0);

/// adds an application manifest to an executable
pub fn manifest<P: AsRef<Utf8Path>>(path: P) {
    let path = path.as_ref();
    assert!(path.exists(), "Path does not exist");

    let output_dir = var("OUT_DIR").unwrap();
    let build_res_file = format!("{output_dir}/manifest{}.rc", CURRENT_MANIFEST_ID.load(Ordering::Relaxed));

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&build_res_file)
        .unwrap();
    let resource_script_content = MANIFEST_RESOURCE_SCRIPT
        .replace(
            "[PATH]",
            &path.as_str().replace('\\', "/"),
        )
        .replace("[ID]", &format!("manifest{}", CURRENT_MANIFEST_ID.load(Ordering::Relaxed)));
    
    CURRENT_MANIFEST_ID.fetch_add(1, Ordering::Relaxed);

    assert_eq!(
        resource_script_content.len(),
        file.write(resource_script_content.as_bytes()).unwrap(),
        "An error occurred while writing the resource file."
    );

    super::link::link(build_res_file);
}
