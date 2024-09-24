use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<ZbAndroidNotification<R>> {
  Ok(ZbAndroidNotification(app.clone()))
}

/// Access to the zb-android-notification APIs.
pub struct ZbAndroidNotification<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ZbAndroidNotification<R> {
  pub fn send_notification(&self, payload: SendNotificationRequest) -> crate::Result<SendNotificationResponse> {
    Ok(SendNotificationResponse {
      value: payload.value,
    })
  }
}
