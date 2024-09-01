// --- error_handling
use std::fmt;
pub enum AppErrors {
    IAmAlright,
    GenericError,
    VaildPath,
    ValidPartList,
    ValidChapterList,
    ValidSceneList,
    ValidPartIndex,
    ReadableFile,
    PlainTextString,
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppErrors::IAmAlright => write!(f, "OK01 - Not An Error."),
            AppErrors::VaildPath => write!(f, "ER01 - Expected valid Content Path"),
            AppErrors::ValidPartList => write!(f, "ER02 - Expected valid list of Book Parts"),
            AppErrors::PlainTextString => write!(f, "ER03 - Expected title to be Plain Text"),
            AppErrors::ValidChapterList => write!(f, "ER04 - Expected valid list of Chapters"),
            AppErrors::ValidSceneList => write!(f, "ER05 - Expected valid list of Scenes"),
            AppErrors::ValidPartIndex => write!(f, "ER06 - That Part of the Book cannot be found"),
            AppErrors::ReadableFile => write!(f, "ER07 - Should have been able to read the file"),
            _ => write!(f, "ER13 - Generic Error - Write Better Code"),
        }
    }
}

// --- error_handling
