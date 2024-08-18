// --- error_handling
pub enum AppErrors {
    IAmAlright,
    GenericError,
    NoVaildPath,
    NoSuchPart,
}

pub fn getErr(inc_err: AppErrors) -> String {
    let user_friendly = match inc_err {
        AppErrors::IAmAlright => "OK01 - Not An Error.",
        AppErrors::NoVaildPath => "ER01 - Expect valid path",
        _ => "ER13 - Generic Error - Write Better Code",
    };

    user_friendly.into()
}
// --- error_handling
