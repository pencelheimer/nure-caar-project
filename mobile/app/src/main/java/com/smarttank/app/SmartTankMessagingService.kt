package com.smarttank.app

import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Intent
import android.util.Log
import androidx.core.app.NotificationCompat
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import com.smarttank.app.data.api.ApiClient
import com.smarttank.app.data.model.RegisterPushTokenRequest
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.launch

class SmartTankMessagingService : FirebaseMessagingService() {

    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())

    override fun onDestroy() {
        scope.cancel()
        super.onDestroy()
    }

    override fun onNewToken(token: String) {
        super.onNewToken(token)
        Log.i(TAG, "FCM token: $token")
        val jwt = SessionManager(this).token ?: return
        scope.launch {
            runCatching {
                ApiClient.api.registerPushToken("Bearer $jwt", RegisterPushTokenRequest(token))
            }.onFailure {
                Log.w(TAG, "Failed to upload new FCM token: $it")
            }
        }
    }

    override fun onMessageReceived(message: RemoteMessage) {
        super.onMessageReceived(message)

        val data = message.data
        if (data["type"] == "alert") {
            handleAlertMessage(data)
            return
        }

        // Fallback: plain notification field (if sent that way)
        message.notification?.let { n ->
            showNotification(
                title = n.title ?: getString(R.string.app_name),
                body = n.body ?: "",
                reservoirId = -1,
            )
        }
    }

    private fun handleAlertMessage(data: Map<String, String>) {
        val reservoirId = data["reservoir_id"]?.toIntOrNull() ?: -1
        val reservoirName = data["reservoir_name"] ?: return
        val conditionType = data["condition_type"] ?: return
        val threshold = data["threshold"]?.toDoubleOrNull() ?: return
        val value = data["value"]?.toDoubleOrNull() ?: return

        val conditionLabel = when (conditionType) {
            "less_than"    -> getString(R.string.condition_below)
            "greater_than" -> getString(R.string.condition_above)
            else           -> conditionType
        }

        val body = getString(
            R.string.push_alert_body,
            reservoirName,
            "%.1f".format(value),
            conditionLabel,
            threshold.toInt(),
        )

        showNotification(
            title = getString(R.string.app_name),
            body = body,
            reservoirId = reservoirId,
        )
    }

    private fun showNotification(title: String, body: String, reservoirId: Int) {
        val intent = Intent(this, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_SINGLE_TOP
            if (reservoirId > 0) putExtra("reservoir_id", reservoirId)
        }
        val pendingIntent = PendingIntent.getActivity(
            this, reservoirId,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
        )

        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setSmallIcon(R.drawable.ic_notification)
            .setContentTitle(title)
            .setContentText(body)
            .setStyle(NotificationCompat.BigTextStyle().bigText(body))
            .setAutoCancel(true)
            .setContentIntent(pendingIntent)
            .build()

        val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        manager.notify(System.currentTimeMillis().toInt(), notification)
    }

    companion object {
        const val CHANNEL_ID = "smarttank_alerts"
        private const val TAG = "SmartTankFCM"
    }
}
