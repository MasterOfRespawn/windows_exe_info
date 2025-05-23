use std::sync::atomic::Ordering;
#[cfg(feature = "versioninfo")]
use crate::versioninfo::*;

#[cfg(feature = "versioninfo")]
const FORMATTED_VERSIONINFO: &str = r#"// This resource script was autogenerated
// Do not change manually!!!
#include<winver.h>
VS_VERSION_INFO VERSIONINFO
FILEVERSION     0, 1, 2, 3
PRODUCTVERSION  4, 5, 6, 7
FILEFLAGSMASK   VS_FFI_FILEFLAGSMASK
FILEFLAGS       VS_FF_SPECIALBUILD
FILEOS          VOS_NT
FILETYPE        VFT_APP
FILESUBTYPE     0
BEGIN
 BLOCK "StringFileInfo"
 BEGIN
  BLOCK "040904E4"
  BEGIN
   VALUE "CompanyName", "TEST\0"
   VALUE "FileDescription", "NONE\0"
   VALUE "FileVersion", "1.2.3.4RC5\0"
   VALUE "InternalName", "6.7.8.9RC0\0"
   VALUE "OriginalFilename", "test1234\0"
   VALUE "ProductName", "TestingApplication\0"
   VALUE "ProductVersion", "To be decided\0"
   VALUE "SpecialBuild", "Made for testing, who would have guessed\0"
  END
 END

 BLOCK "VarFileInfo"
 BEGIN
  VALUE "Translation", 0x0409, 1252
 END
END
"#;

#[cfg(feature = "versioninfo")]
#[test]
fn format_version_info() {
    // initialization
    let mut temp_file = std::env::temp_dir();
    std::env::set_var("OUT_DIR", temp_file.clone());

    let rc = VersionInfo {
        file_version: Version(0, 1, 2, 3),
        product_version: Version(4, 5, 6, 7),
        file_flag_mask: FileFlagMask::Win16,
        file_flags: FileFlags {
            debug: false,
            patched: false,
            prerelease: false,
            privatebuild: false,
            infoinferred: false,
            specialbuild: true,
        },
        file_os: FileOS::NT,
        file_type: FileType::App,
        file_info: vec![FileInfo {
            lang: Language::USEnglish,
            charset: CharacterSet::Multilingual,
            comment: None,
            company_name: "TEST".into(),
            file_description: "NONE".into(),
            file_version: "1.2.3.4RC5".into(),
            internal_name: "6.7.8.9RC0".into(),
            legal_copyright: None,
            legal_trademarks: None,
            original_filename: "test1234".into(),
            product_name: "TestingApplication".into(),
            product_version: "To be decided".into(),
            private_build: None,
            special_build: Some("Made for testing, who would have guessed".into()),
        }],
    };
    // check formatting
    assert_eq!(rc.to_string(), FORMATTED_VERSIONINFO);

    // check double linking prevention
    assert!(!HAS_LINKED_VERSION_INFO.load(Ordering::Relaxed));
    rc.link().unwrap();
    assert!(HAS_LINKED_VERSION_INFO.load(Ordering::Relaxed));
    assert!(rc.link().is_err());

    // cleanup

    temp_file.push("info.rc");
    assert!(temp_file.exists());
    std::fs::remove_file(&temp_file).unwrap();
    assert!(temp_file.pop());

    #[cfg(feature = "embed_resource")]
    {
        temp_file.push("libinfo.a");
        assert!(temp_file.exists());
        std::fs::remove_file(&temp_file).unwrap();
        assert!(temp_file.pop());
    }
    #[cfg(not(feature = "embed_resource"))]
    {
        temp_file.push("info.rc.a");
        assert!(temp_file.exists());
        std::fs::remove_file(&temp_file).unwrap();
        assert!(temp_file.pop());
    }
}

use crate::icon::*;

#[cfg(feature = "icon_placeholder")]
#[test]
fn multi_icon_id() {
    // initialization
    const ITERATIONS: u16 = 4;

    let mut temp_file = std::env::temp_dir();
    std::env::set_var("OUT_DIR", temp_file.clone());
    std::env::set_var("TARGET", "pe-x86-64");

    for i in 0..ITERATIONS {
        // check
        placeholder();
        temp_file.push(format!("icon{i}.rc"));
        assert!(temp_file.exists());
        std::fs::remove_file(&temp_file).unwrap();
        assert!(temp_file.pop());
        // cleanup
        #[cfg(feature = "embed_resource")]
        {
            temp_file.push(format!("libicon{i}.a"));
            assert!(temp_file.exists());
            std::fs::remove_file(&temp_file).unwrap();
            assert!(temp_file.pop());
        }
        #[cfg(not(feature = "embed_resource"))]
        {
            temp_file.push(format!("icon{i}.rc.a"));
            assert!(temp_file.exists());
            std::fs::remove_file(&temp_file).unwrap();
            assert!(temp_file.pop());
        }
    }

    // cleanup (2)
    temp_file.push("icon.ico");
    std::fs::remove_file(&temp_file).unwrap();

    // check (2)
    assert_eq!(CURRENT_ICON_ID.load(Ordering::Relaxed), ITERATIONS);
}
