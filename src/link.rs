#[cfg(feature = "build_cfg")]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -F [ARCH] -o [OUTPUT] -v";
#[cfg(not(feature = "build_cfg"))]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -o [OUTPUT] -v";
#[cfg(not(feature = "embed_resource"))]
use std::process::Command;

pub fn link(resource_path: String) {
    #[cfg(feature = "embed_resource")]
    embed_resource::compile(resource_path, embed_resource::NONE);

    #[cfg(not(feature = "embed_resource"))]
    {
        let resource_file = resource_path.clone() + ".a";
        let args = WINDRES_COMMAND
            .replace("[INPUT]", resource_path.as_str())
            .replace("[OUTPUT]", resource_file.as_str());

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

        assert!(Command::new("windres")
            .args(args.split(" "))
            .spawn()
            .expect("Execution failed")
            .wait()
            .expect("Execution failed")
            .success());

        #[cfg(target_family = "windows")]
        println!("cargo:rustc-link-arg={resource_file}"); // Tell it to link
    }
}
