package com.plugin.zb_android_notification
import android.content.Context  
import android.app.Activity
import androidx.core.graphics.drawable.IconCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class NotificationArgs {
    val title: String? = null
    val body: String? = null
    val bigTitle: String? = null
    val bigDescription: String? = null
    val bigSummaryText: String? = null
}
@TauriPlugin
class NotificationPlugin(private val activity: Activity): Plugin(activity) {
    private val context = activity.applicationContext

    private val implementation = Example(context)


    @Command
    fun send_notification(invoke: Invoke) {
        val args = invoke.parseArgs(NotificationArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.send_notification(
            args.title ?: "",
            args.body ?: "",
            args.bigTitle ?: "",
            args.bigDescription ?: "",
            args.bigSummaryText ?: ""
        ))
        invoke.resolve(ret)
    }
}
