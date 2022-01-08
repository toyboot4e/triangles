//! Hack for working with `sdl2` + `wgpu` on macOS
//!
//! Thanks: https://github.com/Rust-SDL2/rust-sdl2/issues/1116

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use sdl2::video::Window;

/// sdl2 implements raw-window-handle correctly, but wgpu does not for macOS
/// wgpu wrongly expects an NSView to be provided by raw-window-handle, so we have to do a little more work
pub struct WindowWrapper<'a>(pub &'a Window);

unsafe impl<'a> HasRawWindowHandle for WindowWrapper<'a> {
    #[cfg(not(target_os = "macos"))]
    /// all non-mac platforms work correctly, so return the handle directly
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }

    #[cfg(target_os = "macos")]
    /// do some work on macOS to get the root NSView for the NSWindow returned by sdl2
    fn raw_window_handle(&self) -> RawWindowHandle {
        use objc::{msg_send, runtime::Object, sel, sel_impl};
        use raw_window_handle::AppKitHandle;

        let handle = self.0.raw_window_handle();
        match handle {
            RawWindowHandle::AppKit(handle) => {
                let mut x = AppKitHandle::empty();
                x.ns_window = handle.ns_window;
                x.ns_view = unsafe { msg_send![handle.ns_window as *mut Object, contentView] };
                RawWindowHandle::AppKit(x)
            }
            _ => unreachable!(),
        }
    }
}
