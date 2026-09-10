allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

val newBuildDir: Directory =
    rootProject.layout.buildDirectory
        .dir("../../build")
        .get()
rootProject.layout.buildDirectory.value(newBuildDir)

subprojects {
    val newSubprojectBuildDir: Directory = newBuildDir.dir(project.name)
    project.layout.buildDirectory.value(newSubprojectBuildDir)
}
subprojects {
    project.evaluationDependsOn(":app")
}

// Several plugins' own Android modules (file_picker 8.x, flutter_local_
// notifications, connectivity_plus, device_info_plus, open_filex,
// package_info_plus, permission_handler_android, ...) hardcode an older
// compileSdk in their own android/build.gradle rather than reading
// flutter.compileSdkVersion, so bumping just the app module's compileSdk
// doesn't help them — Gradle's AAR metadata check fails on each one in
// turn (whichever depends on flutter_plugin_android_lifecycle, which
// itself now requires compileSdk 36+) as soon as the previous one is
// fixed by upgrading that specific package. Rather than play whack-a-mole
// bumping individual plugin versions as this recurs, force every plugin
// subproject's compileSdk here — this overrides whatever the plugin
// itself declares, and needs no per-plugin fix again if another one hits
// the same wall in the future.
subprojects {
    val applyCompileSdk: () -> Unit = {
        extensions.findByType<com.android.build.gradle.BaseExtension>()?.let { android ->
            android.compileSdkVersion(36)
        }
    }
    // The evaluationDependsOn(":app") above can cascade into evaluating
    // some plugin subprojects earlier than Gradle's normal per-project
    // order — by the time this block runs for one of them, it may already
    // be fully evaluated, and afterEvaluate throws in that case rather
    // than just running immediately. Apply directly when that's already
    // happened; otherwise defer the normal way.
    if (project.state.executed) {
        applyCompileSdk()
    } else {
        afterEvaluate { applyCompileSdk() }
    }
}

tasks.register<Delete>("clean") {
    delete(rootProject.layout.buildDirectory)
}
