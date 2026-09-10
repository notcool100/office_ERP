package com.adyatech.adya_mobile

import io.flutter.embedding.android.FlutterFragmentActivity

// local_auth's Android implementation requires a FragmentActivity host
// (it shows the biometric prompt via the AndroidX BiometricPrompt API,
// which needs a FragmentManager) — plain FlutterActivity doesn't provide
// one, so biometric unlock would crash on the very first call otherwise.
class MainActivity : FlutterFragmentActivity()
