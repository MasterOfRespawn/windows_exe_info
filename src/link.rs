use camino::Utf8Path;
#[cfg(feature = "build_cfg")]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -F [ARCH] -o [OUTPUT] -v";
#[cfg(not(feature = "build_cfg"))]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -o [OUTPUT] -v";
#[cfg(all(feature = "build_cfg", not(test), not(feature = "embed_resource")))]
use build_cfg::build_cfg;
#[cfg(all(feature = "build_cfg", test))]
use std::cfg as build_cfg;
#[cfg(not(feature = "embed_resource"))]
use std::process::Command;

pub fn link<P: AsRef<Utf8Path>>(resource_path: P) {
    let resource_path = resource_path.as_ref();
    #[cfg(feature = "windows_only")]
    if let Err(error) = std::env::var("CARGO_CFG_WINDOWS") {
        // quit if variable does not exist as we are not targeting windows
        if error == std::env::VarError::NotPresent {
            return;
        }

        panic!("Unexpected error {error} while checking for windows target");
    }
    
    #[cfg(feature = "embed_resource")]
    embed_resource::compile(resource_path, embed_resource::NONE).manifest_optional().unwrap();

    #[cfg(all(not(feature = "embed_resource"), feature = "build_cfg"))]
    {
        let resource_file = format!("{resource_path}.a");
        let args = WINDRES_COMMAND
            .replace("[INPUT]", resource_path.as_str())
            .replace("[OUTPUT]", &resource_file);

        #[cfg(feature = "build_cfg")]
        let args = if build_cfg!(target_os = "windows") {
            if build_cfg!(target_pointer_width = "64") {
                args.replace("[ARCH]", "pe-x86-64")
            } else {
                args.replace("[ARCH]", "pe-i386")
            }
        } else {
            panic!("Invalid target operating system");
        };

        #[cfg(any(windows, not(feature = "build_cfg")))]
        let cmd = "windres";
        #[cfg(not(any(windows, not(feature = "build_cfg"))))]
        let cmd = if build_cfg!(target_pointer_width = "64") {
            "x86_64-w64-mingw32-windres"
        } else {
            "i686-w64-mingw32-windres"
        };

        assert!(Command::new(cmd)
            .args(args.split(" "))
            .spawn()
            .expect("Execution failed")
            .wait()
            .expect("Execution failed")
            .success());

        println!("cargo:rustc-link-arg={resource_file}"); // Tell it to link
    }
}
