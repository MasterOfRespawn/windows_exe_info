use std::{fs::OpenOptions, collections::HashMap, io::Write};

use crate::versioninfo::{Version, FileInfo, FileFlags, FileOS, FileType, Language, CharacterSet};

#[test]
fn write_rc_script(){
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(".\\test.rc")
        .unwrap();
    let rc = crate::versioninfo::VersionInfo{
        file_version: Version(0,1,2,3),
        product_version: Version(4,5,6,7),
        file_flag_mask: crate::versioninfo::FileFlagMask::Win16,
        file_flags: FileFlags{
            debug: false,
            patched: false,
            prerelease: false,
            privatebuild: false,
            infoinferred: false,
            specialbuild: true,
        },
        file_os: FileOS::NTWindows32,
        file_type: FileType::App,
        file_info: vec![FileInfo{
            lang: Language::USEnglish,
            charset: CharacterSet::Multilingual,
            comment: None,
            company_name: "TEST".into(),
            file_description:"NONE".into(),
            file_version:"1.2.3.4RC5".into(),
            internal_name:"6.7.8.9RC0".into(),
            legal_copyright:None,
            legal_trademarks:None,
            original_filename:"test1234".into(),
            product_name:"TestingApplication".into(),
            product_version:"To be decided".into(),
            private_build:None,
            special_build:Some("Made for testing, who would have guessed".into()),
            custom: HashMap::new(),
        },],
    };
    f.write(format!("{}", rc).as_bytes()).unwrap();
}