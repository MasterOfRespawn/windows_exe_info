use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use camino::Utf8Path;

const MANIFEST_RESOURCE_SCRIPT: &str = "#define RT_MANIFEST 24
[ID] RT_MANIFEST \"[PATH]\"\n";
pub(crate) static mut CURRENT_MANIFEST_ID: u16 = 0;

/// adds an application manifest to an executable
pub fn manifest<P: AsRef<Utf8Path>>(path: P) {
    let path = path.as_ref();
    assert!(path.exists(), "Path does not exist");

    let output_dir = var("OUT_DIR").unwrap();
    let buildres_file = unsafe { format!("{output_dir}/manifest{}.rc", CURRENT_MANIFEST_ID) };

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&buildres_file)
        .unwrap();
    let resource_script_content = MANIFEST_RESOURCE_SCRIPT
        .replace(
            "[PATH]",
            &path.as_str().replace('\\', "/"),
        )
        .replace("[ID]", &unsafe { format!("manifest{CURRENT_MANIFEST_ID}") });
    unsafe {
        CURRENT_MANIFEST_ID += 1;
    }

    assert_eq!(
        resource_script_content.len(),
        file.write(resource_script_content.as_bytes()).unwrap(),
        "An error occurred while writing the resource file."
    );

    super::link::link(buildres_file);
}
