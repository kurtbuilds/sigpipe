
#[cfg(unix)]
pub fn reset() {
    unsafe {
        ::libc::signal(::libc::SIGPIPE, ::libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
pub fn reset() {
}