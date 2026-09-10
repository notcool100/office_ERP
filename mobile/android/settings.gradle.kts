pluginManagement {
    val flutterSdkPath =
        run {
            val properties = java.util.Properties()
            file("local.properties").inputStream().use { properties.load(it) }
            val flutterSdkPath = properties.getProperty("flutter.sdk")
            require(flutterSdkPath != null) { "flutter.sdk not set in local.properties" }
            flutterSdkPath
        }

    includeBuild("$flutterSdkPath/packages/flutter_tools/gradle")

    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

plugins {
    id("dev.flutter.flutter-plugin-loader") version "1.0.0"
    id("com.android.application") version "9.0.1" apply false
    id("org.jetbrains.kotlin.android") version "2.3.20" apply false
    // Push notifications: once you've dropped android/app/google-services.json
    // in (from your Firebase project), uncomment this line AND the
    // `apply(plugin = "com.google.gms.google-services")` at the bottom of
    // android/app/build.gradle.kts. Left commented out so the project keeps
    // building for anyone who hasn't set up Firebase yet.
    // id("com.google.gms.google-services") version "4.4.2" apply false
}

include(":app")
