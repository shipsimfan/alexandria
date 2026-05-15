use std::{path::PathBuf, ptr::null_mut};
use win32::{
    CoTaskMemFree, shlobj_core::SHGetKnownFolderPath, shtypes::KNOWNFOLDERID, try_hresult,
};

mod cache;
mod desktop;
mod documents;
mod downloads;
mod game_data;
mod home;
mod music;
mod pictures;
mod screenshots;
mod temp;
mod videos;

pub use cache::cache_dir;
pub use desktop::desktop_dir;
pub use documents::documents_dir;
pub use downloads::downloads_dir;
pub use game_data::game_data_dir;
pub use home::home_dir;
pub use music::music_dir;
pub use pictures::pictures_dir;
pub use screenshots::screenshots_dir;
pub use temp::temp_dir;
pub use videos::videos_dir;

/// Get the path to a known folder by its [`KNOWNFOLDERID`]
fn get_known_folder_path(folder_id: &KNOWNFOLDERID) -> Option<PathBuf> {
    let mut path_ptr = null_mut();
    try_hresult!(SHGetKnownFolderPath(
        folder_id,
        0,
        null_mut(),
        &mut path_ptr
    ))
    .map_err(|_| {
        unsafe { CoTaskMemFree(path_ptr.cast()) };
    })
    .ok()?;

    let length = (0..)
        .take_while(|&i| unsafe { *path_ptr.add(i) } != 0)
        .count();
    let slice = unsafe { std::slice::from_raw_parts(path_ptr, length) };
    let path = String::from_utf16(slice).map(PathBuf::from).ok();

    unsafe { CoTaskMemFree(path_ptr.cast()) };

    path
}
