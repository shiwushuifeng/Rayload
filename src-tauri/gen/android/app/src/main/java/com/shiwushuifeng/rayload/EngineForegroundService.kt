package com.shiwushuifeng.rayload

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.os.Build
import android.os.IBinder

/** Keeps the native aria2 engine alive while Rayload is in the background. */
class EngineForegroundService : Service() {
  private val notificationId = 29100

  override fun onCreate() {
    super.onCreate()
    val manager = getSystemService(NotificationManager::class.java)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      // Replace the legacy low-importance channel so the persistent engine
      // notification stays readable across Android skins.
      manager.deleteNotificationChannel(LEGACY_CHANNEL_ID)
      manager.createNotificationChannel(
        NotificationChannel(
          CHANNEL_ID,
          getString(R.string.engine_channel_name),
          NotificationManager.IMPORTANCE_DEFAULT,
        ).apply {
          description = getString(R.string.engine_channel_description)
          setShowBadge(false)
          setSound(null, null)
          enableVibration(false)
          lockscreenVisibility = Notification.VISIBILITY_PUBLIC
        },
      )
      manager.deleteNotificationChannel(LEGACY_QUIET_CHANNEL_ID)
      manager.createNotificationChannel(
        NotificationChannel(
          QUIET_CHANNEL_ID,
          getString(R.string.engine_quiet_channel_name),
          NotificationManager.IMPORTANCE_MIN,
        ).apply {
          description = getString(R.string.engine_quiet_channel_description)
          setShowBadge(false)
          setSound(null, null)
          enableVibration(false)
          lockscreenVisibility = Notification.VISIBILITY_SECRET
        },
      )
    }
  }

  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    when (intent?.action) {
      ACTION_SET_PAUSED -> {
        setPausedState(intent.getBooleanExtra(EXTRA_PAUSED, false))
        publishNotification()
      }
      ACTION_SET_QUIET -> {
        setQuietState(intent.getBooleanExtra(EXTRA_QUIET, false))
        publishNotification()
      }
      else -> publishNotification()
    }
    return START_NOT_STICKY
  }

  private fun publishNotification() {
    val paused = isPaused()
    val quiet = isQuiet()
    val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      Notification.Builder(this, if (quiet) QUIET_CHANNEL_ID else CHANNEL_ID)
    } else {
      @Suppress("DEPRECATION")
      Notification.Builder(this)
    }

    val openApp = PendingIntent.getActivity(
      this,
      0,
      Intent(this, MainActivity::class.java).apply {
        flags = Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP
      },
      PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
    )
    builder
      .setContentTitle(getString(R.string.app_name))
      .setContentText(getString(if (paused) R.string.engine_paused else R.string.engine_running))
      .setSmallIcon(R.drawable.ic_stat_rayload)
      .setContentIntent(openApp)
      .setOngoing(true)
      .setOnlyAlertOnce(true)
      .setCategory(Notification.CATEGORY_SERVICE)
      .setVisibility(Notification.VISIBILITY_PUBLIC)
      .setPriority(Notification.PRIORITY_DEFAULT)

    builder.setSound(null).setDefaults(0)
    val notification = builder.build()

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
      startForeground(notificationId, notification, ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC)
    } else {
      startForeground(notificationId, notification)
    }
  }

  private fun isPaused(): Boolean =
    getSharedPreferences(PREFS, Context.MODE_PRIVATE).getBoolean(KEY_PAUSED, false)

  private fun setPausedState(paused: Boolean) {
    getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().putBoolean(KEY_PAUSED, paused).apply()
  }

  private fun isQuiet(): Boolean =
    getSharedPreferences(PREFS, Context.MODE_PRIVATE).getBoolean(KEY_QUIET, false)

  private fun setQuietState(quiet: Boolean) {
    getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().putBoolean(KEY_QUIET, quiet).apply()
  }

  override fun onTaskRemoved(rootIntent: Intent?) {
    stopSelf()
    super.onTaskRemoved(rootIntent)
  }

  override fun onDestroy() {
    super.onDestroy()
  }

  override fun onBind(intent: Intent?): IBinder? = null

  companion object {
    private const val PREFS = "rayload-engine"
    private const val CHANNEL_ID = "rayload-engine-v2"
    private const val LEGACY_CHANNEL_ID = "rayload-engine"
    private const val QUIET_CHANNEL_ID = "rayload-engine-quiet"
    private const val LEGACY_QUIET_CHANNEL_ID = "rayload-engine-quiet-legacy"
    private const val KEY_RPC_PORT = "rpcPort"
    private const val KEY_RPC_SECRET = "rpcSecret"
    private const val KEY_PAUSED = "paused"
    private const val KEY_QUIET = "quiet"
    private const val ACTION_SET_PAUSED = "com.shiwushuifeng.rayload.action.SET_PAUSED"
    private const val ACTION_SET_QUIET = "com.shiwushuifeng.rayload.action.SET_QUIET"
    private const val EXTRA_PAUSED = "paused"
    private const val EXTRA_QUIET = "quiet"

    @JvmStatic
    fun setCredentials(context: Context, port: Int, secret: String) {
      context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit()
        .putInt(KEY_RPC_PORT, port)
        .putString(KEY_RPC_SECRET, secret)
        .apply()
    }

    @JvmStatic
    fun setPaused(context: Context, paused: Boolean) {
      val intent = Intent(context, EngineForegroundService::class.java)
        .setAction(ACTION_SET_PAUSED)
        .putExtra(EXTRA_PAUSED, paused)
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
        context.startForegroundService(intent)
      } else {
        context.startService(intent)
      }
    }

    @JvmStatic
    fun setQuiet(context: Context, quiet: Boolean) {
      val intent = Intent(context, EngineForegroundService::class.java)
        .setAction(ACTION_SET_QUIET)
        .putExtra(EXTRA_QUIET, quiet)
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
        context.startForegroundService(intent)
      } else {
        context.startService(intent)
      }
    }
  }
}
