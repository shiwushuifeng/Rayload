# JNI entry points and WebView bridges are resolved by name at runtime.
-keep class com.shiwushuifeng.rayload.MainActivity { *; }
-keep class com.shiwushuifeng.rayload.EngineForegroundService { *; }
-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}

-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile
