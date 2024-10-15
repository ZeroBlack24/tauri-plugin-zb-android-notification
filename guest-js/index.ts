import { invoke } from "@tauri-apps/api/core";

interface SendNotificationRequest {
  title: String;
  body?: String;
  bigTitle?: String;
  bigDescription?: String;
  bigSummaryText?: String;
}

export async function send_notification(
  value: SendNotificationRequest
): Promise<string | null> {
  return await invoke<{ value?: string }>(
    "plugin:zb-android-notification|send_notification",
    {
      payload: {
        ...value,
      },
    }
  ).then((r) => (r.value ? r.value : null));
}
