#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatMode {
    Dev,
    Prod,
}

static mut CURRENT_MODE: FormatMode = FormatMode::Dev;

pub fn set_mode(mode: FormatMode) {
    unsafe {
        CURRENT_MODE = mode;
    }
}

pub fn get_mode() -> FormatMode {
    unsafe { CURRENT_MODE }
}
