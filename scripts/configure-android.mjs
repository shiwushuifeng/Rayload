#!/usr/bin/env node
/** Applies the reproducible post-init Android settings Tauri's template omits. */

import { copyFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'

const root = resolve(import.meta.dirname, '..')
const androidRoot = resolve(root, 'src-tauri/gen/android')
const manifestPath = resolve(androidRoot, 'app/src/main/AndroidManifest.xml')
const gradlePath = resolve(androidRoot, 'app/build.gradle.kts')
const activityPath = resolve(androidRoot, 'app/src/main/java/com/shiwushuifeng/rayload/MainActivity.kt')
const servicePath = resolve(androidRoot, 'app/src/main/java/com/shiwushuifeng/rayload/EngineForegroundService.kt')

for (const path of [manifestPath, gradlePath]) {
  if (!existsSync(path)) {
    throw new Error(`Android project is not initialized: ${path}`)
  }
}

const permissions = [
  'android.permission.INTERNET',
  'android.permission.ACCESS_NETWORK_STATE',
  'android.permission.POST_NOTIFICATIONS',
  'android.permission.WAKE_LOCK',
  'android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS',
  'android.permission.VIBRATE',
  'android.permission.MANAGE_EXTERNAL_STORAGE',
  'android.permission.READ_EXTERNAL_STORAGE',
  'android.permission.WRITE_EXTERNAL_STORAGE',
  'android.permission.FOREGROUND_SERVICE',
  'android.permission.FOREGROUND_SERVICE_DATA_SYNC',
]

const protocols = ['magnet', 'ed2k', 'thunder', 'rayload']
const protocolFilters = protocols
  .map(
    (scheme) => `            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <category android:name="android.intent.category.DEFAULT" />
                <category android:name="android.intent.category.BROWSABLE" />
                <data android:scheme="${scheme}" />
            </intent-filter>`,
  )
  .join('\n')

let manifest = readFileSync(manifestPath, 'utf8')

if (!manifest.includes('xmlns:tools=')) {
  manifest = manifest.replace('<manifest ', '<manifest xmlns:tools="http://schemas.android.com/tools" ')
}

const missingPermissions = permissions.filter((permission) => !manifest.includes(permission))
if (missingPermissions.length > 0) {
  manifest = manifest.replace(
    /(<manifest\b[^>]*>)/,
    `$1\n${missingPermissions.map((permission) => `    <uses-permission android:name="${permission}" />`).join('\n')}`,
  )
}

if (!manifest.includes('android:scheme="magnet"')) {
  manifest = manifest.replace(/(<activity\b[^>]*>)/, `$1\n${protocolFilters}`)
}

if (!manifest.includes('android:mimeType="text/plain"')) {
  manifest = manifest.replace(
    /(<activity\b[^>]*>)/,
    `$1\n            <intent-filter>
                <action android:name="android.intent.action.SEND" />
                <category android:name="android.intent.category.DEFAULT" />
                <data android:mimeType="text/plain" />
            </intent-filter>`,
  )
}

if (!manifest.includes('android:largeHeap="true"')) {
  manifest = manifest.replace('<application\n', '<application\n        android:largeHeap="true"\n')
}

if (!manifest.includes('android:windowSoftInputMode=')) {
  manifest = manifest.replace(
    /(<activity\b[^>]*android:name="\.MainActivity")/,
    '$1\n            android:windowSoftInputMode="adjustResize"',
  )
}

manifest = manifest.replace(/\n\s*android:extractNativeLibs="[^"]*"/g, '')

if (!manifest.includes('EngineForegroundService')) {
  manifest = manifest.replace(
    '    </application>',
    `        <service
            android:name=".EngineForegroundService"
            android:exported="false"
            android:foregroundServiceType="dataSync"
            android:stopWithTask="true" />
    </application>`,
  )
}

writeFileSync(manifestPath, manifest)

if (!existsSync(activityPath)) {
  copyFileSync(resolve(import.meta.dirname, 'android/MainActivity.kt'), activityPath)
}
if (!existsSync(servicePath)) {
  copyFileSync(resolve(import.meta.dirname, 'android/EngineForegroundService.kt'), servicePath)
}

let gradle = readFileSync(gradlePath, 'utf8')
gradle = gradle.replace(
  'namespace = "dev.aninsomniacy.rayburst"',
  'namespace = "com.shiwushuifeng.rayload"',
)
gradle = gradle.replace(
  'applicationId = "dev.aninsomniacy.rayburst"',
  'applicationId = "com.shiwushuifeng.rayload"',
)
gradle = gradle.replace(
  'manifestPlaceholders["usesCleartextTraffic"] = "false"',
  'manifestPlaceholders["usesCleartextTraffic"] = "true"',
)

if (!gradle.includes('useLegacyPackaging = true')) {
  gradle = gradle.replace(
    '    buildTypes {',
    `    packaging {
        jniLibs {
            useLegacyPackaging = true
        }
    }
    buildTypes {`,
  )
}

if (!gradle.includes('signingConfigs {')) {
  gradle = gradle.replace(
    '    buildTypes {',
    `    signingConfigs {
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
    buildTypes {`,
  )
}

if (!gradle.includes('signingConfig = signingConfigs.getByName("release")')) {
  gradle = gradle.replace(
    'getByName("release") {',
    `getByName("release") {
            if (rootProject.file("../../../signing/keystore.properties").exists()) {
                signingConfig = signingConfigs.getByName("release")
            }`,
  )
}

if (!gradle.includes('"prepareAria2Engine"')) {
  gradle += `

val aria2EngineSource = rootProject.file("../../../binaries/aria2-next-aarch64-linux-android")
val aria2EngineDir = layout.projectDirectory.dir("src/main/jniLibs/arm64-v8a")

tasks.register<Copy>("prepareAria2Engine") {
    from(aria2EngineSource)
    into(aria2EngineDir)
    rename { "libaria2.so" }
}

tasks.named("preBuild") {
    dependsOn("prepareAria2Engine")
}
`
}

gradle = gradle.replace('rename { "libaria2-next.so" }', 'rename { "libaria2.so" }')

gradle = gradle.replace(
  `tasks.named<Copy>("prepareAria2Engine") {
    doLast {
        delete(aria2EngineDir.file("libaria2-next.so"))
    }
}`,
  '',
)

if (!gradle.includes('"removeLegacyAria2EngineAlias"')) {
  gradle += `

tasks.register<Delete>("removeLegacyAria2EngineAlias") {
    delete(layout.projectDirectory.file("src/main/jniLibs/arm64-v8a/libaria2-next.so"))
}

tasks.named("prepareAria2Engine") {
    finalizedBy("removeLegacyAria2EngineAlias")
}
`
}

gradle = gradle.replace(
  'rootProject.file("../../../binaries/aria2-next-aarch64-linux-android")',
  'rootProject.file("../../binaries/aria2-next-aarch64-linux-android")',
)
writeFileSync(gradlePath, gradle)

console.log('Configured Android manifest, API 37 build, and local RPC networking.')
