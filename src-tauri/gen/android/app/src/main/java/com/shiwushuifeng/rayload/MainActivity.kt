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
import android.os.Handler
import android.os.Looper
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
import androidx.appcompat.app.AlertDialog
import androidx.core.content.FileProvider
import java.io.File
import java.lang.ref.WeakReference
import java.net.Inet4Address
import java.net.NetworkInterface
import java.util.Collections
import org.json.JSONObject

class MainActivity : TauriActivity() {
  private var activeWebView: WebView? = null
  private var pendingNewDownload = false
  private var backCallback: OnBackPressedCallback? = null
  private var lastBackAt = 0L
  private val shareQueueFile by lazy { File(filesDir, "rayload-share-pending.txt") }
  private val directoryPicker = registerForActivityResult(
    ActivityResultContracts.OpenDocumentTree(),
  ) { uri -> handleDirectorySelection(uri) }

  companion object {
    private val VIRTUAL_INTERFACE_PREFIXES =
      listOf("tun", "tap", "ppp", "rmnet", "clat", "ip6tnl", "dummy", "v4-", "ipsec", "p2p")
    private const val EXTERNAL_STORAGE_DOCUMENTS = "com.android.externalstorage.documents"
    private const val APP_DOWNLOAD_DIRECTORY = "Download/Rayload"
    private const val ACTION_NEW_DOWNLOAD = "com.shiwushuifeng.rayload.action.NEW_DOWNLOAD"
    private const val REQUEST_NOTIFICATION_PERMISSION = 4102
    private const val REQUEST_LEGACY_STORAGE_PERMISSION = 4103
    private const val WAKE_LOCK_TAG = "Rayload:active-downloads"
    private const val LAUNCHER_ICON_PREFS = "rayload-launcher-icon"
    private const val CLIPBOARD_PREFS = "rayload-permissions"
    private const val KEY_CLIPBOARD_CONSENT = "clipboard-consent"

    private var downloadWakeLock: PowerManager.WakeLock? = null
    private var activeActivity: WeakReference<MainActivity>? = null

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
        "blue" -> ".LauncherBlue"
        "teal" -> ".LauncherTeal"
        else -> return false
      }
      val packageManager = context.packageManager
      val choices = listOf(".LauncherPurple", ".LauncherGold", ".LauncherBlue", ".LauncherTeal")
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
    @Suppress("UNUSED_PARAMETER")
    fun releaseDownloadWakeLock(context: Context): Boolean {
      val wakeLock = downloadWakeLock ?: return true
      if (wakeLock.isHeld) wakeLock.release()
      downloadWakeLock = null
      return true
    }

    /** Stops the background service, finishes the task, and exits the process. */
    @Keep
    @JvmStatic
    fun shutdownApplication(context: Context): Boolean {
      releaseDownloadWakeLock(context)
      runCatching {
        context.stopService(Intent(context, EngineForegroundService::class.java))
      }
      val activity = (context as? Activity) ?: activeActivity?.get()
      if (activity != null) {
        activity.runOnUiThread { activity.finishAffinity() }
      }
      Handler(Looper.getMainLooper()).postDelayed({
        Process.killProcess(Process.myPid())
        System.exit(0)
      }, 400)
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
    activeActivity = WeakReference(this)
    startEngineService()
    captureShareIntent(intent)
    captureNewDownloadIntent(intent)
  }

  override fun onNewIntent(intent: Intent) {
    super.onNewIntent(intent)
    setIntent(intent)
    captureShareIntent(intent)
    captureNewDownloadIntent(intent)
  }

  override fun onResume() {
    super.onResume()
    dispatchNativeEvent("rayload-permission-state-changed")
  }

  override fun onDestroy() {
    backCallback?.remove()
    backCallback = null
    activeWebView = null
    if (activeActivity?.get() === this) activeActivity = null
    if (isFinishing) releaseDownloadWakeLock(this)
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
    if (pendingNewDownload) {
      webView.postDelayed({
        if (pendingNewDownload) dispatchNativeEvent("rayload-new-task-requested")
      }, 800)
    }
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
    fun setEngineCredentials(port: Int, secret: String) {
      EngineForegroundService.setCredentials(this@MainActivity, port, secret)
    }

    @JavascriptInterface
    fun setEnginePaused(paused: Boolean) {
      EngineForegroundService.setPaused(this@MainActivity, paused)
    }

    @JavascriptInterface
    fun setEngineNotificationQuiet(quiet: Boolean) {
      EngineForegroundService.setQuiet(this@MainActivity, quiet)
    }

    /** Keep-awake dashboard: clears automatically when the activity is destroyed. */
    @JavascriptInterface
    fun setKeepScreenOn(enabled: Boolean) {
      runOnUiThread { this@MainActivity.window.decorView.keepScreenOn = enabled }
    }

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
    fun consumeNewDownloadRequest(): Boolean {
      val pending = pendingNewDownload
      pendingNewDownload = false
      return pending
    }

    @JavascriptInterface
    fun requestBatteryOptimizationExemption() {
      runOnUiThread { this@MainActivity.requestBatteryOptimizationExemption() }
    }

    @JavascriptInterface
    fun clipboardPermissionStatus(): String = currentClipboardPermissionStatus()

    @JavascriptInterface
    fun requestClipboardPermission() {
      runOnUiThread { this@MainActivity.requestClipboardPermission() }
    }

    @JavascriptInterface
    fun installPermissionStatus(): String = currentInstallPermissionStatus()

    @JavascriptInterface
    fun requestInstallPermission() {
      runOnUiThread { this@MainActivity.requestInstallPermission() }
    }

    /** Restarts the app in a fresh process so persisted preferences re-apply. */
    @JavascriptInterface
    fun restartApp() {
      runOnUiThread { this@MainActivity.restartApp() }
    }

    /** Active non-loopback IPv4 address, used by the LAN panel. */
    @JavascriptInterface
    fun localIpAddress(): String = currentLocalIpAddress()
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

  private fun captureNewDownloadIntent(intent: Intent?) {
    if (intent?.action != ACTION_NEW_DOWNLOAD) return
    intent.action = null
    pendingNewDownload = true
    if (activeWebView != null) dispatchNativeEvent("rayload-new-task-requested")
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

  /**
   * Android has no clipboard runtime permission; reading is allowed only while
   * the app is in the foreground. This explicit consent gates Rayload's
   * foreground-only clipboard link detection.
   */
  private fun currentClipboardPermissionStatus(): String = if (
    getSharedPreferences(CLIPBOARD_PREFS, Context.MODE_PRIVATE)
      .getBoolean(KEY_CLIPBOARD_CONSENT, false)
  ) {
    "granted"
  } else {
    "denied"
  }

  private fun requestClipboardPermission() {
    if (currentClipboardPermissionStatus() == "granted") {
      dispatchNativeEvent("rayload-permission-state-changed")
      return
    }
    AlertDialog.Builder(this)
      .setTitle(R.string.clipboard_permission_title)
      .setMessage(R.string.clipboard_permission_message)
      .setPositiveButton(R.string.clipboard_permission_allow) { _, _ ->
        getSharedPreferences(CLIPBOARD_PREFS, Context.MODE_PRIVATE)
          .edit()
          .putBoolean(KEY_CLIPBOARD_CONSENT, true)
          .apply()
        dispatchNativeEvent("rayload-permission-state-changed")
      }
      .setNegativeButton(R.string.clipboard_permission_deny) { _, _ ->
        dispatchNativeEvent("rayload-permission-state-changed")
      }
      .show()
  }

  private fun currentInstallPermissionStatus(): String {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) return "granted"
    return try {
      if (packageManager.canRequestPackageInstalls()) "granted" else "denied"
    } catch (_: Exception) {
      "unavailable"
    }
  }

  private fun requestInstallPermission() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
      dispatchNativeEvent("rayload-permission-state-changed")
      return
    }
    val intent = Intent(
      Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
      Uri.parse("package:$packageName"),
    )
    if (!startActivitySafely(this, intent)) {
      startActivitySafely(this, Intent(Settings.ACTION_SECURITY_SETTINGS))
    }
  }

  /** Relaunches the launcher activity before terminating this process. */
  private fun restartApp() {
    val intent = packageManager.getLaunchIntentForPackage(packageName)?.apply {
      addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
    }
    if (intent != null) startActivity(intent)
    Handler(Looper.getMainLooper()).postDelayed({ Process.killProcess(Process.myPid()) }, 120)
  }

  /**
   * Returns a private IPv4 address that a computer on the same LAN can reach.
   * Wi-Fi, hotspot and Ethernet interfaces win; VPN, cellular and other
   * virtual links are skipped so the QR code never advertises a dead endpoint.
   */
  private fun currentLocalIpAddress(): String {
    return try {
      findLanIpv4 { name -> name.startsWith("wlan") || name.startsWith("ap") || name.startsWith("eth") }
    } catch (_: Exception) {
      ""
    }
  }

  private fun findLanIpv4(acceptInterface: (String) -> Boolean): String {
    val interfaces = NetworkInterface.getNetworkInterfaces() ?: return ""
    for (networkInterface in Collections.list(interfaces)) {
      if (!networkInterface.isUp || networkInterface.isLoopback) continue
      val interfaceName = networkInterface.name.lowercase()
      if (VIRTUAL_INTERFACE_PREFIXES.any { interfaceName.startsWith(it) }) continue
      if (!acceptInterface(interfaceName)) continue
      for (address in Collections.list(networkInterface.inetAddresses)) {
        if (address is Inet4Address && !address.isLoopbackAddress && !address.isLinkLocalAddress &&
          isPrivateIpv4(address)
        ) {
          address.hostAddress?.takeIf { it.isNotBlank() }?.let { return it }
        }
      }
    }
    return ""
  }

  private fun isPrivateIpv4(address: Inet4Address): Boolean {
    val octets = address.address
    if (octets.size != 4) return false
    val first = octets[0].toInt() and 0xFF
    val second = octets[1].toInt() and 0xFF
    return first == 10 || (first == 172 && second in 16..31) || (first == 192 && second == 168)
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
