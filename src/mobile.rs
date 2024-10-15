use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_zb_android_notification);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<ZbAndroidNotification<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.zb_android_notification", "NotificationPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_zb_android_notification)?;
  Ok(ZbAndroidNotification(handle))
}

/// Access to the zb-android-notification APIs.
pub struct ZbAndroidNotification<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> ZbAndroidNotification<R> {
  pub fn send_notification(&self, payload: SendNotificationRequest) -> crate::Result<SendNotificationResponse> {
    self
      .0
      .run_mobile_plugin("send_notification", payload)
      .map_err(Into::into)
  }
}
