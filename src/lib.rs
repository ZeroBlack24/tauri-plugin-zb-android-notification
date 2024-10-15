use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ZbAndroidNotification;
#[cfg(mobile)]
use mobile::ZbAndroidNotification;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the zb-android-notification APIs.
pub trait ZbAndroidNotificationExt<R: Runtime> {
  fn zb_android_notification(&self) -> &ZbAndroidNotification<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ZbAndroidNotificationExt<R> for T {
  fn zb_android_notification(&self) -> &ZbAndroidNotification<R> {
    self.state::<ZbAndroidNotification<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("zb-android-notification")
    .invoke_handler(tauri::generate_handler![commands::send_notification])
    .setup(|app, api| {
      #[cfg(mobile)]
      let zb_android_notification = mobile::init(app, api)?;


      #[cfg(desktop)]
      let zb_android_notification = desktop::init(app, api)?;

      
      app.manage(zb_android_notification);
      Ok(())
    })
    .build()
}
