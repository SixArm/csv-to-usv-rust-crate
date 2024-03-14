pub fn u8_to_log_level(u8: u8) -> Option<::log::Level> {
    match u8 {
        1 => Some(::log::Level::Error),
        2 => Some(::log::Level::Warn),
        3 => Some(::log::Level::Info),
        4 => Some(::log::Level::Debug),
        5 => Some(::log::Level::Trace),
        _ => None,
    }
}
