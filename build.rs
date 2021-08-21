fn main() {
    windows::build! {
      Windows::Win32::UI::Shell::{IThumbnailProvider, SHChangeNotify, WTS_ALPHATYPE},
      Windows::Win32::Graphics::Gdi::{CreateBitmap, DeleteObject, HBITMAP},
      Windows::Win32::Storage::StructuredStorage::{ISequentialStream, IStream},
      Windows::Win32::System::PropertiesSystem::IInitializeWithStream,
      Windows::Win32::Foundation::{WINCODEC_ERR_BADIMAGE, WINCODEC_ERR_WRONGSTATE},
    };
}
