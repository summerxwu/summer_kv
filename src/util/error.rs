
pub struct ErrorInfo(u16, &'static str);
pub const ERROR_BOARD: &'static [ErrorInfo] = &[
    ErrorInfo(0, "block overflow"),
    ErrorInfo(1, ""),
    ErrorInfo(2, ""),
];

