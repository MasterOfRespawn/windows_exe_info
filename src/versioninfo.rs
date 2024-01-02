//! Structs for adding version information to an executable
use std::env::var;
use std::fmt::Write as FmtWrite;
use std::fs::OpenOptions;
use std::io::Write;

pub(crate) static mut HAS_LINKED_VERSIONINFO: bool = false;

/// The main wrapper struct.
/// Implements custom formatting converting it into an rc script.
/// Only one versioninfo struct can be used per executable.
pub struct VersionInfo {
    pub file_version: Version,
    pub product_version: Version,
    pub file_flag_mask: FileFlagMask,
    pub file_flags: FileFlags,
    pub file_os: FileOS,
    pub file_type: FileType,
    pub file_info: Vec<FileInfo>,
}

impl core::fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        writeln!(result, "// This resource script was autogenerated")?;
        writeln!(result, "// Do not change manually!!!")?;
        writeln!(result, "#include<winver.h>")?;
        writeln!(result, "VS_VERSION_INFO VERSIONINFO")?;
        writeln!(result, "FILEVERSION     {}", self.file_version)?;
        writeln!(result, "PRODUCTVERSION  {}", self.product_version)?;
        writeln!(result, "FILEFLAGSMASK   {}", self.file_flag_mask)?;
        writeln!(result, "FILEFLAGS       {}", self.file_flags)?;
        writeln!(result, "FILEOS          {}", self.file_os)?;
        writeln!(result, "FILETYPE        {}", self.file_type)?;
        writeln!(result, "FILESUBTYPE     {}", self.file_type.get_subvalue())?;

        if !self.file_info.is_empty() {
            writeln!(result, "BEGIN")?;
            writeln!(result, r#" BLOCK "StringFileInfo""#)?;
            writeln!(result, " BEGIN")?;
            for file_info in &self.file_info {
                writeln!(
                    result,
                    r#"  BLOCK "{}{}""#,
                    file_info.lang.to_hex(),
                    file_info.charset.to_hex()
                )?;
                writeln!(result, "  BEGIN")?;
                writeln!(
                    result,
                    r#"   VALUE "CompanyName", {}"#,
                    file_info.company_name
                )?;
                writeln!(
                    result,
                    r#"   VALUE "FileDescription", {}"#,
                    file_info.file_description
                )?;
                writeln!(
                    result,
                    r#"   VALUE "FileVersion", {}"#,
                    file_info.file_version
                )?;
                writeln!(
                    result,
                    r#"   VALUE "InternalName", {}"#,
                    file_info.internal_name
                )?;
                writeln!(
                    result,
                    r#"   VALUE "OriginalFilename", {}"#,
                    file_info.original_filename
                )?;
                writeln!(
                    result,
                    r#"   VALUE "ProductName", {}"#,
                    file_info.product_name
                )?;
                writeln!(
                    result,
                    r#"   VALUE "ProductVersion", {}"#,
                    file_info.product_version
                )?;
                if let Some(t) = &file_info.comment {
                    writeln!(result, r#"   VALUE "Comments", {t}"#)?;
                }
                if let Some(t) = &file_info.legal_copyright {
                    writeln!(result, r#"   VALUE "LegalCopyright", {t}"#)?;
                }
                if let Some(t) = &file_info.legal_trademarks {
                    writeln!(result, r#"   VALUE "LegalTrademarks", {t}"#)?;
                }
                if let Some(t) = &file_info.private_build {
                    writeln!(result, r#"   VALUE "PrivateBuild", {t}"#)?;
                }
                if let Some(t) = &file_info.special_build {
                    writeln!(result, r#"   VALUE "SpecialBuild", {t}"#)?;
                }
                writeln!(result, "  END")?;
            }
            writeln!(result, " END")?;
            writeln!(result)?;
            writeln!(result, r#" BLOCK "VarFileInfo""#)?;
            writeln!(result, " BEGIN")?;
            write!(result, r#"  VALUE "Translation""#)?;
            for file_info in &self.file_info {
                write!(
                    result,
                    ", 0x{}, {}",
                    file_info.lang.to_hex(),
                    file_info.charset.to_decimal()
                )?;
            }
            writeln!(result)?;
            writeln!(result, " END")?;
        }
        writeln!(result, "END")?;
        write!(f, "{result}")
    }
}
impl VersionInfo {
    /// Writes the content of the struct into a file and tries to compile and link it
    /// panics if it is invoked more than once
    pub fn link(&self) -> Result<(), &str> {
        if unsafe { HAS_LINKED_VERSIONINFO } {
            return Err("Only one versioninfo can be linked");
        }
        let output_dir = var("OUT_DIR").unwrap();
        let buildres_file = format!("{output_dir}/info.rc");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&buildres_file)
            .unwrap();
        let resource_script_content = self.to_string();

        assert_eq!(
            resource_script_content.len(),
            file.write(resource_script_content.as_bytes()).unwrap(),
            "An error occurred while writing the resource file."
        );

        super::link::link(buildres_file);
        unsafe { HAS_LINKED_VERSIONINFO = true };
        Ok(())
    }

    /// creates the `VersionInfo` struct from cargo environment variables.
    /// sets the `FileInfo` Language to English and without the optional fields
    /// `comment`, `company_name`, `legal_copyright` and `legal_trademarks`
    pub fn from_cargo_env() -> Self {
        Self::from_cargo_env_ex(None, None, None, None)
    }

    /// creates the `VersionInfo` struct from cargo environment variables.
    /// sets the `FileInfo` Language to English including the optional fields
    /// `comment`, `company_name`, `legal_copyright` and `legal_trademarks`
    /// according to user input
    pub fn from_cargo_env_ex(
        comment: Option<&str>,
        company_name: Option<&str>,
        legal_copyright: Option<&str>,
        legal_trademarks: Option<&str>,
    ) -> Self {
        let version = Version(
            var("CARGO_PKG_VERSION_MAJOR")
                .ok()
                .and_then(|major| major.parse().ok())
                .unwrap_or_default(),
            var("CARGO_PKG_VERSION_MINOR")
                .ok()
                .and_then(|minor| minor.parse().ok())
                .unwrap_or_default(),
            var("CARGO_PKG_VERSION_PATCH")
                .ok()
                .and_then(|patch| patch.parse().ok())
                .unwrap_or_default(),
            var("CARGO_PKG_VERSION_PRE")
                .ok()
                .and_then(|pre| pre.parse().ok())
                .unwrap_or_default(),
        );
        Self {
            file_version: version,
            product_version: version,
            file_flag_mask: FileFlagMask::Win16,
            file_flags: FileFlags {
                debug: var("PROFILE").unwrap_or_default() != "release",
                patched: false,
                prerelease: false,
                privatebuild: false,
                infoinferred: false,
                specialbuild: false,
            },
            file_os: FileOS::Windows32,
            file_type: FileType::App,
            file_info: vec![FileInfo {
                lang: Language::USEnglish,
                charset: CharacterSet::Multilingual,
                comment: comment.map(<&str>::into),
                company_name: company_name.map(<&str>::into).unwrap_or_default(),
                file_description: var("CARGO_PKG_DESCRIPTION").unwrap_or_default().into(),
                file_version: var("CARGO_PKG_VERSION").unwrap_or_default().into(),
                internal_name: var("CARGO_PKG_NAME").unwrap_or_default().into(),
                legal_copyright: legal_copyright.map(<&str>::into),
                legal_trademarks: legal_trademarks.map(<&str>::into),
                original_filename: format!("{}.exe", var("CARGO_PKG_NAME").unwrap_or_default())
                    .into(),
                product_name: var("CARGO_PKG_NAME").unwrap_or_default().into(),
                product_version: var("CARGO_PKG_VERSION").unwrap_or_default().into(),
                private_build: None,
                special_build: None,
            }],
        }
    }
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self::from_cargo_env()
    }
}

/// autogenerates and links version information from cargo environment variables
///
/// panics if linking fails
pub fn link_cargo_env() {
    VersionInfo::from_cargo_env().link().unwrap();
}

/// Representation of the STRINGFILEINFO block in a versioninfo struct.
/// Can be used multiple times in the main VERSIONINFO block
/// for different languages
pub struct FileInfo {
    pub lang: Language,
    pub charset: CharacterSet,
    ///Additional information that should be displayed for diagnostic purposes.
    pub comment: Option<RCString>,
    ///Company that produced the file—for example, "Microsoft Corporation" or "Standard Microsystems Corporation, Inc." This string is required.
    pub company_name: RCString,
    ///File description to be presented to users. This string may be displayed in a list box when the user is choosing files to install—for example, "Keyboard Driver for AT-Style Keyboards". This string is required.
    pub file_description: RCString,
    ///Version number of the file—for example, "3.10" or "5.00.RC2". This string is required.
    pub file_version: RCString,
    ///Internal name of the file, if one exists—for example, a module name if the file is a dynamic-link library. If the file has no internal name, this string should be the original filename, without extension. This string is required.
    pub internal_name: RCString,
    ///Copyright notices that apply to the file. This should include the full text of all notices, legal symbols, copyright dates, and so on. This string is optional.
    pub legal_copyright: Option<RCString>,
    ///Trademarks and registered trademarks that apply to the file. This should include the full text of all notices, legal symbols, trademark numbers, and so on. This string is optional.
    pub legal_trademarks: Option<RCString>,
    ///Original name of the file, not including a path. This information enables an application to determine whether a file has been renamed by a user. The format of the name depends on the file system for which the file was created. This string is required.
    pub original_filename: RCString,
    ///Name of the product with which the file is distributed. This string is required.
    pub product_name: RCString,
    ///Version of the product with which the file is distributed—for example, "3.10" or "5.00.RC2". This string is required.
    pub product_version: RCString,
    ///Information about a private version of the file—for example, "Built by TESTER1 on \TESTBED". This string should be present only if VS_FF_PRIVATEBUILD is specified in the fileflags parameter of the root block.
    pub private_build: Option<RCString>,
    ///Text that specifies how this version of the file differs from the standard version—for example, "Private build for TESTER1 solving mouse problems on M250 and M250E computers". This string should be present only if VS_FF_SPECIALBUILD is specified in the fileflags parameter of the root block.
    pub special_build: Option<RCString>,
    //pub custom: HashMap<String, RCString>,
}

/// The language for the `FileInfo` struct
pub enum Language {
    Arabic,                //0x0401
    Polish,                //0x0415
    Bulgarian,             //0x0402
    PortugueseBrazil,      //0x0416
    Catalan,               //0x0403
    RhaetoRomanic,         //0x0417
    TraditionalChinese,    //0x0404
    Romanian,              //0x0418
    Czech,                 //0x0405
    Russian,               //0x0419
    Danish,                //0x0406
    CroatoSerbianLatin,    //0x041A
    German,                //0x0407
    Slovak,                //0x041B
    Greek,                 //0x0408
    Albanian,              //0x041C
    USEnglish,             //0x0409
    Swedish,               //0x041D
    CastilianSpanish,      //0x040A
    Thai,                  //0x041E
    Finnish,               //0x040B
    Turkish,               //0x041F
    French,                //0x040C
    Urdu,                  //0x0420
    Hebrew,                //0x040D
    Bahasa,                //0x0421
    Hungarian,             //0x040E
    SimplifiedChinese,     //0x0804
    Icelandic,             //0x040F
    SwissGerman,           //0x0807
    Italian,               //0x0410
    UKEnglish,             //0x0809
    Japanese,              //0x0411
    SpanishMexico,         //0x080A
    Korean,                //0x0412
    BelgianFrench,         //0x080C
    Dutch,                 //0x0413
    CanadianFrench,        //0x0C0C
    NorwegianBokmal,       //0x041
    SwissFrench,           //0x100C
    SwissItalian,          //0x0810
    PortuguesePortugal,    //0x0816
    BelgianDutch,          //0x0813
    SerboCyrillicCroatian, //0x081A
    NorwegianNynorsk,      //0x0814
}

impl Language {
    pub const fn to_hex(&self) -> &str {
        match self {
            Self::Arabic => "0401",
            Self::Polish => "0415",
            Self::Bulgarian => "0402",
            Self::PortugueseBrazil => "0416",
            Self::Catalan => "0403",
            Self::RhaetoRomanic => "0417",
            Self::TraditionalChinese => "0404",
            Self::Romanian => "0418",
            Self::Czech => "0405",
            Self::Russian => "0419",
            Self::Danish => "0406",
            Self::CroatoSerbianLatin => "041A",
            Self::German => "0407",
            Self::Slovak => "041B",
            Self::Greek => "0408",
            Self::Albanian => "041C",
            Self::USEnglish => "0409",
            Self::Swedish => "041D",
            Self::CastilianSpanish => "040A",
            Self::Thai => "041E",
            Self::Finnish => "040B",
            Self::Turkish => "041F",
            Self::French => "040C",
            Self::Urdu => "0420",
            Self::Hebrew => "040D",
            Self::Bahasa => "0421",
            Self::Hungarian => "040E",
            Self::SimplifiedChinese => "0804",
            Self::Icelandic => "040F",
            Self::SwissGerman => "0807",
            Self::Italian => "0410",
            Self::UKEnglish => "0809",
            Self::Japanese => "0411",
            Self::SpanishMexico => "080A",
            Self::Korean => "0412",
            Self::BelgianFrench => "080C",
            Self::Dutch => "0413",
            Self::CanadianFrench => "0C0C",
            Self::NorwegianBokmal => "041",
            Self::SwissFrench => "100C",
            Self::SwissItalian => "810",
            Self::PortuguesePortugal => "0816",
            Self::BelgianDutch => "813",
            Self::SerboCyrillicCroatian => "081A",
            Self::NorwegianNynorsk => "0814",
        }
    }
}

/// The character set for the `FileInfo` struct
pub enum CharacterSet {
    ASCII7bit,             // 0 	0000
    JapanShiftJISX0208,    // 932 	03A4
    KoreaShiftKSC5601,     // 949 	03B5
    TaiwanBig5,            // 950 	03B6
    Unicode,               // 1200 	04B0
    Latin2EasternEuropean, // 1250 	04E2
    Cyrillic,              // 1251 	04E3
    Multilingual,          // 1252 	04E4
    Greek,                 // 1253 	04E5
    Turkish,               // 1254 	04E6
    Hebrew,                // 1255 	04E7
    Arabic,                // 1256 	04E8
}

impl CharacterSet {
    pub const fn to_hex(&self) -> &str {
        match self {
            Self::ASCII7bit => "0000",
            Self::JapanShiftJISX0208 => "03A4",
            Self::KoreaShiftKSC5601 => "03B5",
            Self::TaiwanBig5 => "03B6",
            Self::Unicode => "04B0",
            Self::Latin2EasternEuropean => "04E2",
            Self::Cyrillic => "04E3",
            Self::Multilingual => "04E4",
            Self::Greek => "04E5",
            Self::Turkish => "04E6",
            Self::Hebrew => "04E7",
            Self::Arabic => "04E8",
        }
    }

    pub const fn to_decimal(&self) -> &str {
        match self {
            Self::ASCII7bit => "0",
            Self::JapanShiftJISX0208 => "932",
            Self::KoreaShiftKSC5601 => "949",
            Self::TaiwanBig5 => "950",
            Self::Unicode => "1200",
            Self::Latin2EasternEuropean => "1250",
            Self::Cyrillic => "1251",
            Self::Multilingual => "1252",
            Self::Greek => "1253",
            Self::Turkish => "1254",
            Self::Hebrew => "1255",
            Self::Arabic => "1256",
        }
    }
}

/// Wrapper correct string escaping in rc script
#[derive(Default)]
pub struct RCString(String);

impl From<String> for RCString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for RCString {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl core::fmt::Display for RCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}\0""#, self.0)
    }
}

/// wrapper for the actual version, format:
/// major, minor, patch, build
#[derive(Clone, Copy)]
pub struct Version(pub u16, pub u16, pub u16, pub u16);
impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}

/// is always 0x3f
pub enum FileFlagMask {
    Win16, // = 0x3f, // THERE IS ONLY ONE OPTION
    Custom(u32),
}

impl core::fmt::Display for FileFlagMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win16 => write!(f, "VS_FFI_FILEFLAGSMASK"),
            Self::Custom(i) => write!(f, "{i}"),
        }
    }
}

/// special flags descirbing certain attributes (look at flag descriptions)
pub struct FileFlags {
    /// File contains debugging information or is compiled with debugging features enabled.
    pub debug: bool,
    /// File has been modified and is not identical to the original shipping file of the same version number.
    pub patched: bool,
    /// File is a development version, not a commercially released product.
    pub prerelease: bool,
    /// File was not built using standard release procedures. If this value is given, the StringFileInfo block must contain a PrivateBuild string.
    pub privatebuild: bool,
    /// I found this in the gcc winver.h file. I have no idea what this does
    pub infoinferred: bool,
    /// File was built by the original company using standard release procedures but is a variation of the standard file of the same version number. If this value is given, the StringFileInfo block block must contain a SpecialBuild string.
    pub specialbuild: bool,
}

impl core::fmt::Display for FileFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut flags = Vec::new();
        if self.debug {
            flags.push("VS_FF_DEBUG");
        }
        if self.patched {
            flags.push("VS_FF_PRERELEASE");
        }
        if self.prerelease {
            flags.push("VS_FF_PATCHED");
        }
        if self.privatebuild {
            flags.push("VS_FF_PRIVATEBUILD");
        }
        if self.infoinferred {
            flags.push("VS_FF_INFOINFERRED");
        }
        if self.specialbuild {
            flags.push("VS_FF_SPECIALBUILD");
        }
        if flags.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", flags.join(" | "))
        }
    }
}

/// the operating system the application is designed for.
/// the default in the microsoft documentation is Windows32
pub enum FileOS {
    Unknown,      // = 0x00000000,
    Dos,          // = 0x00010000,
    OS216Bit,     // = 0x00020000,
    OS232Bit,     // = 0x00030000,
    NT,           // = 0x00040000,
    WinCE,        // = 0x00050000,
    Base,         // = 0x00000000,
    Windows16,    // = 0x00000001,
    PM16,         // = 0x00000002,
    PM32,         // = 0x00000003,
    Windows32,    // = 0x00000004,
    DosWindows16, // = 0x00010001,
    DosWindows32, // = 0x00010004,
    OS216BitPM16, // = 0x00020002,
    OS232BitPM32, // = 0x00030003,
    NTWindows32,  // = 0x00040004,
    /// Use a non defined id
    Custom(u32),
}

impl FileOS {
    fn get_value(&self) -> String {
        match self {
            Self::Unknown => "VOS_UNKNOWN".to_string(),
            Self::Dos => "VOS_DOS".to_string(),
            Self::OS216Bit => "VOS_OS216".to_string(),
            Self::OS232Bit => "VOS_OS232".to_string(),
            Self::NT => "VOS_NT".to_string(),
            Self::WinCE => "VOS_WINCE".to_string(),
            Self::Base => "VOS__BASE".to_string(),
            Self::Windows16 => "VOS__WINDOWS16".to_string(),
            Self::PM16 => "VOS__PM16".to_string(),
            Self::PM32 => "VOS__PM32".to_string(),
            Self::Windows32 => "VOS__WINDOWS32".to_string(),
            Self::DosWindows16 => "VOS_DOS_WINDOWS16".to_string(),
            Self::DosWindows32 => "VOS_DOS_WINDOWS32".to_string(),
            Self::OS216BitPM16 => "VOS_OS216_PM16".to_string(),
            Self::OS232BitPM32 => "VOS_OS232_PM32".to_string(),
            Self::NTWindows32 => "VOS_NT_WINDOWS32".to_string(),
            Self::Custom(i) => i.to_string(),
        }
    }
}

impl core::fmt::Display for FileOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

/// The file type
pub enum FileType {
    Unknown,               // = 0x00000000,
    App,                   // = 0x00000001,
    Dll,                   // = 0x00000002,
    Driver(SubTypeDriver), // = 0x00000003,
    Font(SubTypeFont),     // = 0x00000004,
    VXD,                   // = 0x00000005,
    StaticLibrary,         // = 0x00000007,
    Custom(u32, u32),
}

impl FileType {
    fn get_value(&self) -> String {
        match self {
            Self::Unknown => "VFT_UNKNOWN".to_string(),
            Self::App => "VFT_APP".to_string(),
            Self::Dll => "VFT_DLL".to_string(),
            Self::Driver(_) => "VFT_DRV".to_string(),
            Self::Font(_) => "VFT_FONT".to_string(),
            Self::VXD => "VFT_VXD".to_string(),
            Self::StaticLibrary => "VFT_STATIC_LIB".to_string(),
            Self::Custom(i, _) => i.to_string(),
        }
    }

    fn get_subvalue(&self) -> String {
        match self {
            Self::Driver(t) => t.get_value(),
            Self::Font(t) => t.get_value(),
            Self::Custom(_, i) => i.to_string(),
            _ => "0".to_string(),
        }
    }
}

impl core::fmt::Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

/// file subtype for driver
pub enum SubTypeDriver {
    Unknown,          // = 0x00000000,
    Printer,          // = 0x00000001,
    Keyboard,         // = 0x00000002,
    Language,         // = 0x00000003,
    Display,          // = 0x00000004,
    Mouse,            // = 0x00000005,
    Network,          // = 0x00000006,
    System,           // = 0x00000007,
    Installable,      // = 0x00000008,
    Sound,            // = 0x00000009,
    Comm,             // = 0x0000000A,
    InputMethod,      // = 0x0000000B,
    VersionedPrinter, // = 0x0000000C,
    Custom(u32),
}

impl SubTypeDriver {
    fn get_value(&self) -> String {
        match self {
            Self::Unknown => "VFT2_UNKNOWN".to_string(),
            Self::Printer => "VFT2_DRV_PRINTER".to_string(),
            Self::Keyboard => "VFT2_DRV_KEYBOARD".to_string(),
            Self::Language => "VFT2_DRV_LANGUAGE".to_string(),
            Self::Display => "VFT2_DRV_DISPLAY".to_string(),
            Self::Mouse => "VFT2_DRV_MOUSE".to_string(),
            Self::Network => "VFT2_DRV_NETWORK".to_string(),
            Self::System => "VFT2_DRV_SYSTEM".to_string(),
            Self::Installable => "VFT2_DRV_INSTALLABLE".to_string(),
            Self::Sound => "VFT2_DRV_SOUND".to_string(),
            Self::Comm => "VFT2_DRV_COMM".to_string(),
            Self::InputMethod => "VFT2_DRV_INPUTMETHOD".to_string(),
            Self::VersionedPrinter => "VFT2_DRV_VERSIONED_PRINTER".to_string(),
            Self::Custom(i) => i.to_string(),
        }
    }
}

impl core::fmt::Display for SubTypeDriver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

/// file subtype for fonts
pub enum SubTypeFont {
    RasterFont,   // = 0x00000001,
    VectorFont,   // = 0x00000002,
    TrueTypeFont, // = 0x00000003,
    Custom(u32),
}

impl SubTypeFont {
    fn get_value(&self) -> String {
        match self {
            Self::RasterFont => "VFT2_FONT_RASTER".to_string(),
            Self::VectorFont => "VFT2_FONT_VECTOR".to_string(),
            Self::TrueTypeFont => "VFT2_FONT_TRUETYPE".to_string(),
            Self::Custom(i) => i.to_string(),
        }
    }
}

impl core::fmt::Display for SubTypeFont {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
