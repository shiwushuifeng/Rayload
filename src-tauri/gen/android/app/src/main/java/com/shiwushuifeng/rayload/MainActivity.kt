package com.shiwushuifeng.rayload

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.ClipData
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.os.Environment
import android.os.PowerManager
import android.os.Process
import android.os.SystemClock
import android.provider.DocumentsContract
import android.provider.Settings
import android.webkit.JavascriptInterface
import android.webkit.MimeTypeMap
import android.webkit.WebView
import androidx.activity.OnBackPressedCallback
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.annotation.Keep
import androidx.core.content.FileProvider
import java.io.File
import org.json.JSONObject

class MainActivity : TauriActivity() {
  private var activeWebView: WebView? = null
  private var backCallback: OnBackPressedCallback? = null
  private var lastBackAt = 0L
  private val shareQueueFile by lazy { File(filesDir, "rayload-share-pending.txt") }
  private val directoryPicker = registerForActivityResult(
    ActivityResultContracts.OpenDocumentTree(),
  ) { uri -> handleDirectorySelection(uri) }

  companion object {
    private const val EXTERNAL_STORAGE_DOCUMENTS = "com.android.externalstorage.documents"
    private const val APP_DOWNLOAD_DIRECTORY = "Download/Rayload"
    private const val REQUEST_NOTIFICATION_PERMISSION = 4102
    private const val REQUEST_LEGACY_STORAGE_PERMISSION = 4103
    private const val WAKE_LOCK_TAG = "Rayload:active-downloads"
    private const val LAUNCHER_ICON_PREFS = "rayload-launcher-icon"

    private var downloadWakeLock: PowerManager.WakeLock? = null

    /** Shares a downloaded file through a correctly scoped FileProvider URI. */
    @Keep
    @JvmStatic
    fun shareFile(context: Context, rawPath: String): Boolean {
      val file = File(rawPath)
      if (!file.isFile || !file.canRead()) return false
      val uri = fileProviderUri(context, file) ?: return false
      val mime = context.contentResolver.getType(uri)
        ?: MimeTypeMap.getSingleton().getMimeTypeFromExtension(file.extension.lowercase())
        ?: "*/*"
      val send = Intent(Intent.ACTION_SEND).apply {
        type = mime
        setDataAndType(uri, mime)
        putExtra(Intent.EXTRA_STREAM, uri)
        clipData = ClipData.newUri(context.contentResolver, file.name, uri)
        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
      }
      val chooser = Intent.createChooser(send, "Share ${file.name}").apply {
        addFlags(
          Intent.FLAG_GRANT_READ_URI_PERMISSION or
            Intent.FLAG_ACTIVITY_NEW_TASK,
        )
      }
      return startActivitySafely(context, chooser)
    }

    @Keep
    @JvmStatic
    fun openFile(context: Context, rawPath: String): Boolean {
      val file = File(rawPath)
      if (!file.isFile || !file.canRead()) return false
      val uri = fileProviderUri(context, file) ?: return false
      val extension = file.extension.lowercase()
      val mime = MimeTypeMap.getSingleton().getMimeTypeFromExtension(extension)
        ?: context.contentResolver.getType(uri)?.takeUnless {
          it == "application/octet-stream" || it == "*/*"
        }
        ?: return false
      val intent = Intent(Intent.ACTION_VIEW).apply {
        setDataAndType(uri, mime)
        clipData = ClipData.newUri(context.contentResolver, file.name, uri)
        addFlags(
          Intent.FLAG_GRANT_READ_URI_PERMISSION or
            Intent.FLAG_ACTIVITY_NEW_TASK,
        )
      }
      return startActivitySafely(context, intent)
    }

    /** Switches the Android home-screen shortcut between bundled icon aliases. */
    @Keep
    @JvmStatic
    fun setAppIcon(context: Context, style: String): Boolean {
      val selected = when (style) {
        "gold" -> ".LauncherGold"
        "purple" -> ".LauncherPurple"
        else -> return false
      }
      val packageManager = context.packageManager
      val choices = listOf(".LauncherPurple", ".LauncherGold")
      return runCatching {
        val selectedComponent = ComponentName(context.packageName, context.packageName + selected)
        val preferred = PackageManager.COMPONENT_ENABLED_STATE_ENABLED
        val disabled = PackageManager.COMPONENT_ENABLED_STATE_DISABLED
        val flags = PackageManager.DONT_KILL_APP
        packageManager.setComponentEnabledSetting(selectedComponent, preferred, flags)
        choices
          .filter { it != selected }
          .forEach { suffix ->
            packageManager.setComponentEnabledSetting(
              ComponentName(context.packageName, context.packageName + suffix),
              disabled,
              flags,
            )
          }
        context.getSharedPreferences(LAUNCHER_ICON_PREFS, Context.MODE_PRIVATE)
          .edit()
          .putString("style", style)
          .apply()
        true
      }.getOrDefault(false)
    }

    @Keep
    @JvmStatic
    @SuppressLint("WakelockTimeout")
    fun acquireDownloadWakeLock(context: Context): Boolean {
      val power = context.getSystemService(Context.POWER_SERVICE) as? PowerManager ?: return false
      val wakeLock = downloadWakeLock ?: power.newWakeLock(
        PowerManager.PARTIAL_WAKE_LOCK,
        WAKE_LOCK_TAG,
      ).apply {
        setReferenceCounted(false)
        downloadWakeLock = this
      }
      if (!wakeLock.isHeld) wakeLock.acquire()
      return wakeLock.isHeld
    }

    @Keep
    @JvmStatic
    fun releaseDownloadWakeLock(): Boolean {
      val wakeLock = downloadWakeLock ?: return true
      if (wakeLock.isHeld) wakeLock.release()
      downloadWakeLock = null
      return true
    }

    private fun fileProviderUri(context: Context, file: File): Uri? {
      val canonical = runCatching { file.canonicalFile }.getOrElse { return null }
      return runCatching {
        FileProvider.getUriForFile(
          context,
          "${context.packageName}.fileprovider",
          canonical,
        )
      }.getOrNull()
    }

    private fun startActivitySafely(context: Context, intent: Intent): Boolean =
      runCatching {
        if (context !is Activity) intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        context.startActivity(intent)
        true
      }.getOrDefault(false)

    private fun documentTreePath(uri: Uri): String? {
      val documentId = runCatching { DocumentsContract.getTreeDocumentId(uri) }.getOrNull()
        ?: return null
      val separator = documentId.indexOf(':')
      if (separator < 0) return null
      val storageId = documentId.substring(0, separator)
      val relative = documentId.substring(separator + 1).trimStart('/')
      val root = when (storageId) {
        "primary" -> Environment.getExternalStorageDirectory()
        else -> File("/storage/$storageId")
      }
      if (!root.isDirectory) return null
      return File(root, relative).absolutePath
    }
  }

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    startEngineService()
    captureShareIntent(intent)
  }

  override fun onNewIntent(intent: Intent) {
    super.onNewIntent(intent)
    setIntent(intent)
    captureShareIntent(intent)
  }

  override fun onResume() {
    super.onResume()
    dispatchNativeEvent("rayload-permission-state-changed")
  }

  override fun onDestroy() {
    backCallback?.remove()
    backCallback = null
    activeWebView = null
    if (isFinishing) releaseDownloadWakeLock()
    super.onDestroy()
  }

  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    activeWebView = webView
    webView.settings.apply {
      setSupportZoom(false)
      builtInZoomControls = false
      displayZoomControls = false
      textZoom = 100
    }
    webView.addJavascriptInterface(NativeBridge(), "RayloadNative")
    webView.setDownloadListener { url, userAgent, contentDisposition, _, contentLength ->
      val detail = JSONObject()
        .put("url", url)
        .apply {
          webView.url?.let { put("referer", it) }
          userAgent?.let { put("userAgent", it) }
          contentDisposition?.let { put("contentDisposition", it) }
          if (contentLength > 0) put("contentLength", contentLength)
        }
      dispatchNativeEvent("rayload-download-requested", detail)
    }
    installBackNavigation(webView)
  }

  private fun handleDirectorySelection(uri: Uri?) {
    if (uri == null) {
      dispatchNativeEvent("rayload-directory-selection-cancelled")
      return
    }
    runCatching {
      contentResolver.takePersistableUriPermission(
        uri,
        Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION,
      )
    }
    val path = documentTreePath(uri)
    if (path == null) {
      dispatchNativeEvent(
        "rayload-directory-selection-failed",
        JSONObject().put("message", "Choose a folder on primary or removable storage."),
      )
      return
    }
    dispatchNativeEvent("rayload-directory-selected", JSONObject().put("path", path))
  }

  @Deprecated("Uses the platform request-code callback required by TauriActivity.")
  override fun onRequestPermissionsResult(
    requestCode: Int,
    permissions: Array<out String>,
    grantResults: IntArray,
  ) {
    super.onRequestPermissionsResult(requestCode, permissions, grantResults)
    if (
      requestCode == REQUEST_NOTIFICATION_PERMISSION ||
      requestCode == REQUEST_LEGACY_STORAGE_PERMISSION
    ) {
      dispatchNativeEvent("rayload-permission-state-changed")
    }
  }

  private inner class NativeBridge {
    @JavascriptInterface
    fun lightTap() {
      activeWebView?.performHapticFeedback(android.view.HapticFeedbackConstants.KEYBOARD_TAP)
    }

    @JavascriptInterface
    fun chooseDirectory() {
      runOnUiThread { openDirectoryPicker() }
    }

    @JavascriptInterface
    fun shareFile(path: String): Boolean = Companion.shareFile(this@MainActivity, path)

    @JavascriptInterface
    fun openFile(path: String): Boolean = Companion.openFile(this@MainActivity, path)

    @JavascriptInterface
    fun setAppIcon(style: String): Boolean = Companion.setAppIcon(this@MainActivity, style)

    @JavascriptInterface
    fun storagePermissionStatus(): String = currentStoragePermissionStatus()

    @JavascriptInterface
    fun requestStoragePermission() {
      runOnUiThread { this@MainActivity.requestStoragePermission() }
    }

    @JavascriptInterface
    fun notificationPermissionStatus(): String = currentNotificationPermissionStatus()

    @JavascriptInterface
    fun requestNotificationPermission() {
      runOnUiThread { this@MainActivity.requestNotificationPermission() }
    }

    @JavascriptInterface
    fun batteryOptimizationStatus(): String =
      if (isIgnoringBatteryOptimizations()) "granted" else "denied"

    @JavascriptInterface
    fun requestBatteryOptimizationExemption() {
      runOnUiThread { this@MainActivity.requestBatteryOptimizationExemption() }
    }
  }

  private fun openDirectoryPicker() {
    val initialFolder = File(Environment.getExternalStorageDirectory(), APP_DOWNLOAD_DIRECTORY)
    initialFolder.mkdirs()
    val initialUri = DocumentsContract.buildDocumentUri(
      EXTERNAL_STORAGE_DOCUMENTS,
      "primary:$APP_DOWNLOAD_DIRECTORY",
    )
    val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
      putExtra(DocumentsContract.EXTRA_INITIAL_URI, initialUri)
      addFlags(
        Intent.FLAG_GRANT_READ_URI_PERMISSION or
          Intent.FLAG_GRANT_WRITE_URI_PERMISSION or
          Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION,
      )
    }
    directoryPicker.launch(initialUri)
  }

  private fun captureShareIntent(intent: Intent?) {
    if (intent?.action != Intent.ACTION_SEND) return
    if (intent.type?.startsWith("text/plain") != true) return
    val sharedText = intent.getCharSequenceExtra(Intent.EXTRA_TEXT)?.toString()?.trim()
    if (sharedText.isNullOrEmpty()) return

    shareQueueFile.parentFile?.mkdirs()
    shareQueueFile.appendText("$sharedText\n", Charsets.UTF_8)
    dispatchNativeEvent("rayload-share-requested")
  }

  private fun dispatchNativeEvent(eventName: String, detail: JSONObject = JSONObject()) {
    val webView = activeWebView ?: return
    val script =
      "window.dispatchEvent(new CustomEvent(" +
        "${JSONObject.quote(eventName)},{detail:$detail}))"
    webView.post { webView.evaluateJavascript(script, null) }
  }

  private fun installBackNavigation(webView: WebView) {
    backCallback?.remove()
    val callback = object : OnBackPressedCallback(true) {
      override fun handleOnBackPressed() {
        webView.evaluateJavascript(
          "(window.__rayloadHandleBack && window.__rayloadHandleBack()) || false",
        ) { result ->
          if (result.trim('"') == "true") {
            lastBackAt = 0L
            return@evaluateJavascript
          }

          val now = SystemClock.elapsedRealtime()
          if (lastBackAt > 0L && now - lastBackAt <= 2000L) {
            lastBackAt = 0L
            moveTaskToBack(true)
          } else {
            lastBackAt = now
            dispatchNativeEvent("rayload-back-home")
          }
        }
      }
    }
    backCallback = callback
    onBackPressedDispatcher.addCallback(this, callback)
  }

  private fun currentStoragePermissionStatus(): String {
    val granted = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      Environment.isExternalStorageManager()
    } else {
      checkSelfPermission(Manifest.permission.READ_EXTERNAL_STORAGE) ==
        PackageManager.PERMISSION_GRANTED
    }
    return if (granted) "granted" else "denied"
  }

  private fun requestStoragePermission() {
    if (currentStoragePermissionStatus() == "granted") {
      dispatchNativeEvent("rayload-permission-state-changed")
      return
    }
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      val appSettings = Intent(
        Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION,
        Uri.parse("package:$packageName"),
      )
      if (!startActivitySafely(this, appSettings)) {
        startActivitySafely(
          this,
          Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION),
        )
      }
      return
    }
    requestPermissions(
      arrayOf(
        Manifest.permission.READ_EXTERNAL_STORAGE,
        Manifest.permission.WRITE_EXTERNAL_STORAGE,
      ),
      REQUEST_LEGACY_STORAGE_PERMISSION,
    )
  }

  private fun currentNotificationPermissionStatus(): String {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) return "granted"
    return if (
      checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) ==
      PackageManager.PERMISSION_GRANTED
    ) {
      "granted"
    } else {
      "denied"
    }
  }

  private fun requestNotificationPermission() {
    if (
      Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU ||
      currentNotificationPermissionStatus() == "granted"
    ) {
      dispatchNativeEvent("rayload-permission-state-changed")
      return
    }
    requestPermissions(
      arrayOf(Manifest.permission.POST_NOTIFICATIONS),
      REQUEST_NOTIFICATION_PERMISSION,
    )
  }

  private fun isIgnoringBatteryOptimizations(): Boolean {
    val power = getSystemService(Context.POWER_SERVICE) as? PowerManager ?: return false
    return power.isIgnoringBatteryOptimizations(packageName)
  }

  private fun requestBatteryOptimizationExemption() {
    val request = Intent(
      Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS,
      Uri.parse("package:$packageName"),
    )
    if (startActivitySafely(this, request)) return
    startActivitySafely(this, Intent(Settings.ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS))
  }

  private fun startEngineService() {
    val intent = Intent(this, EngineForegroundService::class.java)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      startForegroundService(intent)
    } else {
      startService(intent)
    }
  }
}
