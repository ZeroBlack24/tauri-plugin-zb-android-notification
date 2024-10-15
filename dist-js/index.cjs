'use strict';

var core = require('@tauri-apps/api/core');

async function send_notification(value) {
    return await core.invoke("plugin:zb-android-notification|send_notification", {
        payload: {
            ...value,
        },
    }).then((r) => (r.value ? r.value : null));
}

exports.send_notification = send_notification;
