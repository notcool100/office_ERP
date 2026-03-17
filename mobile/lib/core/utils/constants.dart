import 'package:flutter/foundation.dart' show kIsWeb, defaultTargetPlatform, TargetPlatform;

class AppConstants {
  static String get apiBaseUrl {
    if (!kIsWeb && defaultTargetPlatform == TargetPlatform.android) {
      return 'http://10.0.2.2:3117';
    }
    // For iOS emulator, Linux, macOS, Windows, and Web
    return 'http://127.0.0.1:3117';
  }

  static String get wsBaseUrl {
    if (!kIsWeb && defaultTargetPlatform == TargetPlatform.android) {
      return 'ws://10.0.2.2:3117/ws';
    }
    return 'ws://127.0.0.1:3117/ws';
  }
}
