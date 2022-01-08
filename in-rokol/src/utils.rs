//! Utilities

pub fn as_bytes<T>(xs: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            xs as *const _ as *const _,
            xs.len() * std::mem::size_of::<T>(),
        )
    }
}
