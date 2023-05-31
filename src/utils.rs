pub fn slice_as_raw<T>(slice: &[T]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * core::mem::size_of::<T>(),
        )
    }
}

pub fn normal_f32_to_u8(val: f32) -> u8 {
    (val * 127.0) as u8 + 127
}
