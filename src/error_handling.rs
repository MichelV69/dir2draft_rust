// --- error_handling
pub enum AppErrors {
    IAmAlright,
    GenericError,
    VaildPath,
    ValidPartList,
    PlainTextString,
}

pub fn getExpected(inc_err: AppErrors) -> String {
    let user_friendly = match inc_err {
        AppErrors::IAmAlright => "OK01 - Not An Error.",
        AppErrors::VaildPath => "ER01 - Expected valid Content Path",
        AppErrors::ValidPartList => "ER02 - Expected valid list of Book Parts",
        AppErrors::PlainTextString => "ER03 - Expected title to be Plain Text",
        _ => "ER13 - Generic Error - Write Better Code",
    };

    user_friendly.into()
}
// --- error_handling
