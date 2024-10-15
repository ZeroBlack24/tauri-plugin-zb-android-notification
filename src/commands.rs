use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::ZbAndroidNotificationExt;

#[command]
pub(crate) async fn send_notification<R: Runtime>(
    app: AppHandle<R>,
    payload: SendNotificationRequest,
) -> Result<SendNotificationResponse> {
    app.zb_android_notification().send_notification(payload)
}
