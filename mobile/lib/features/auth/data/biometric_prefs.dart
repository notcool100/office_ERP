import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Whether the user has opted into a biometric app-lock: once enabled, the
/// app is not usable after backgrounding until Face ID/fingerprint
/// succeeds (their session token stays valid the whole time — this is a
/// local re-entry gate, not a second login).
class BiometricPrefs {
  static const _key = 'biometric_lock_enabled';

  Future<bool> isEnabled() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getBool(_key) ?? false;
  }

  Future<void> setEnabled(bool value) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_key, value);
  }
}

final biometricPrefsProvider = Provider<BiometricPrefs>((ref) => BiometricPrefs());

final biometricLockEnabledProvider = FutureProvider<bool>((ref) {
  return ref.watch(biometricPrefsProvider).isEnabled();
});
