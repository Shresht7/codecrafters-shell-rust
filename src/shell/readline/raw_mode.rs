/// RAII guard to enable terminal raw mode and ensure that it's disabled automatically on drop
pub(super) struct RawModeGuard;

impl RawModeGuard {
    pub(super) fn new() -> std::io::Result<Self> {
        // Enable terminal raw mode
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Ensure raw mode is disabled (even on error) when the guard is dropped
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
