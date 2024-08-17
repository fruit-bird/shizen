use shizen_buffers::prelude::*;
use shizen_macros::shizen;

#[shizen]
pub fn MidSideSwapPlugin(audio_buffer: StereoBuffer) -> StereoBuffer {
    audio_buffer.iter().map(|[l, r]| [r, l]).collect()
}

// make tokio::main type macro to turn main into this entry point
// though the template project has a lib.rs, so just simulate the
// main function here
#[no_mangle]
pub extern "system" fn GetPluginFactory() -> *mut std::ffi::c_void {
    // Return a pointer to your factory implementation
    // Box::into_raw(Box::new(Factory)) as *mut std::ffi::c_void
    todo!()
}
