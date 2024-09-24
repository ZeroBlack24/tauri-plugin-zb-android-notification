use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ZbAndroidNotificationExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.zb_android_notification().ping(payload)
}
