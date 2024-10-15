package com.plugin.zb_android_notification


import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.graphics.drawable.Drawable
import android.os.Build
import android.util.Log
import androidx.annotation.RequiresApi
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.content.ContextCompat
import androidx.core.graphics.drawable.IconCompat


const val packageName = "com.plugin.zb_android_notification"
@RequiresApi(Build.VERSION_CODES.M)
fun showNotification(context: Context,title:String,body:String,bigTitle:String,bigDescription:String,bigSummaryText:String) {
    val channelId = "my_channel_id"
    val notificationId = 1

    // Create the notification channel for Android 8.0 and above
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
        val channelName = "My Channel"
        val channelDescription = "Channel for app notifications"
        val importance = NotificationManager.IMPORTANCE_DEFAULT
        val channel = NotificationChannel(channelId, channelName, importance).apply {
            description = channelDescription
        }

        // Register the channel with the system
        val notificationManager: NotificationManager =
            context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(channel)
    }
    val logoResId = context.applicationInfo.icon
    println(logoResId)
    var appLogo: IconCompat? = null
    appLogo = if (logoResId != 0) {
        IconCompat.createWithResource(context, logoResId);
    }
    else{
        IconCompat.createWithResource(context, android.R.drawable.btn_radio)
    }
    // Build the notification
    val icon: IconCompat = appLogo
    val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        NotificationCompat.Builder(context, channelId)
            .setContentTitle(title)
            .setContentText(body)
            .setPriority(1)
            .setSmallIcon(icon)
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setLargeIcon(icon.toIcon(context))
            .setLights(3,300,300)
            .setVibrate(longArrayOf(1000, 1000, 1000, 1000, 1000))
            .setStyle(NotificationCompat.BigPictureStyle()
                .setBigContentTitle(bigTitle)
                .setSummaryText(bigSummaryText)
                .setContentDescription(bigDescription)
            )


    } else {
        TODO("VERSION.SDK_INT < S")
    }
    // Show the notification
    with(NotificationManagerCompat.from(context)) {
        notify(notificationId, builder.build())
    }
}

class Example(private val context: Context) {

    fun send_notification(title:String,body:String,bigTitle:String,bigDescription:String,bigSummaryText:String): String {

        val context =
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                showNotification(context,title,body,bigTitle,bigDescription,bigSummaryText)

            } else {
                TODO("VERSION.SDK_INT < M")
            }
        Log.i("title", title)
        return title
    }


}
