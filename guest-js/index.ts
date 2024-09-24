import { invoke } from '@tauri-apps/api/core'

export async function send_notification(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:zb-android-notification|send_notification', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}
