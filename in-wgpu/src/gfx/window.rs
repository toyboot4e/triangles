//! SDL2 window handle in handy format for `wgpu`

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use sdl2::video::Window;
use vek::Extent2;

/// SDL2 window wrapper
pub struct WindowWrapper {
    pub sdl: sdl2::Sdl,
    pub vid: sdl2::VideoSubsystem,
    /// Raw SDL2 window
    pub raw: Window,
}

impl WindowWrapper {
    pub fn new(f: &mut dyn FnMut(&sdl2::VideoSubsystem) -> Window) -> Self {
        let sdl = sdl2::init().unwrap();
        let vid = sdl.video().unwrap();
        let window = f(&vid);
        Self {
            sdl,
            vid,
            raw: window,
        }
    }

    /// Frame buffer size in pixels in `f32`
    ///
    /// NOTE: It lies SDL's DPI scaling is disabled.
    pub fn fb_size_f(&self) -> Extent2<f32> {
        self.fb_size_u().as_::<f32>()
    }

    /// Frame buffer size in pixels in `u32`
    ///
    /// NOTE: It lies SDL's DPI scaling is disabled.
    pub fn fb_size_u(&self) -> Extent2<u32> {
        self.raw.drawable_size().into()
    }

    /// Scaling factor from window size to frame buffer
    ///
    /// NOTE: It lies SDL's DPI scaling is disabled.
    pub fn dpi_scale(&self) -> Extent2<f32> {
        self.fb_size_f() / Extent2::<u32>::from(self.raw.size()).as_::<f32>()
    }
}

/// Hack for working with `sdl2` + `wgpu` on macOS
///
/// Thanks: <https://github.com/Rust-SDL2/rust-sdl2/issues/1116>
unsafe impl HasRawWindowHandle for WindowWrapper {
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

        let handle = self.raw.raw_window_handle();
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
