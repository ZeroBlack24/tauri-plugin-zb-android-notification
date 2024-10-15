import { invoke } from '@tauri-apps/api/core';

async function send_notification(value) {
    return await invoke("plugin:zb-android-notification|send_notification", {
        payload: {
            ...value,
        },
    }).then((r) => (r.value ? r.value : null));
}

export { send_notification };
