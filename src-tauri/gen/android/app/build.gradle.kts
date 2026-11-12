import java.util.Properties
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

val includeSmokeX86_64Engine =
    providers.environmentVariable("RAYLOAD_SMOKE_X86_64").orNull == "1"

android {
    compileSdk = 37
    namespace = "com.shiwushuifeng.rayload"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "true"
        applicationId = "com.shiwushuifeng.rayload"
        minSdk = 24
        targetSdk = 37
        versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
        versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
        // All product translations ship through the WebView JSON catalogs.
        // Android library resource locales are not used by the app UI.
        resourceConfigurations += listOf("en", "zh")
    }
    signingConfigs {
        create("release") {
            val keystorePropertiesFile = rootProject.file("../../../signing/keystore.properties")
            if (keystorePropertiesFile.exists()) {
                val keystoreProperties = Properties()
                keystorePropertiesFile.inputStream().use { keystoreProperties.load(it) }
                keyAlias = keystoreProperties.getProperty("keyAlias")
                keyPassword = keystoreProperties.getProperty("password")
                storeFile = rootProject.file("../../../" + keystoreProperties.getProperty("storeFile"))
                storePassword = keystoreProperties.getProperty("password")
            }
        }
    }
    packaging {
        jniLibs {
            useLegacyPackaging = true
            if (!includeSmokeX86_64Engine) {
                excludes.add("x86_64/**")
            }
        }
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            if (rootProject.file("../../../signing/keystore.properties").exists()) {
                signingConfig = signingConfigs.getByName("release")
            }
            isMinifyEnabled = true
            isShrinkResources = true
            optimization {
               enable = true
            }
            proguardFiles(
                *fileTree(".") {
                  include("**/*.pro")
                  exclude("build/**")
                }.files.toTypedArray()
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    buildFeatures {
        buildConfig = true
    }
}

kotlin {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_1_8
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.lifecycle:lifecycle-process:2.10.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = file("tauri.build.gradle.kts"))


val aria2EngineSource = rootProject.file("../../binaries/aria2-next-aarch64-linux-android")
val aria2EngineDir = layout.projectDirectory.dir("src/main/jniLibs/arm64-v8a")

tasks.register<Copy>("prepareAria2Engine") {
    from(aria2EngineSource)
    into(aria2EngineDir)
    rename { "libaria2.so" }
}

tasks.named("preBuild") {
    dependsOn("prepareAria2Engine")
}





tasks.register<Delete>("removeLegacyAria2EngineAlias") {
    delete(layout.projectDirectory.file("src/main/jniLibs/arm64-v8a/libaria2-next.so"))
}

tasks.named("prepareAria2Engine") {
    finalizedBy("removeLegacyAria2EngineAlias")
}

if (includeSmokeX86_64Engine) {
    val aria2X86_64EngineSource =
        rootProject.file("../../../.toolchains/binaries/aria2-next-android-x86_64")
    check(aria2X86_64EngineSource.exists()) {
        "Missing emulator engine: $aria2X86_64EngineSource"
    }
    tasks.register<Copy>("prepareAria2X86_64Engine") {
        from(aria2X86_64EngineSource)
        into(layout.projectDirectory.dir("src/main/jniLibs/x86_64"))
        rename { "libaria2.so" }
    }
    tasks.named("preBuild") {
        dependsOn("prepareAria2X86_64Engine")
    }
}
