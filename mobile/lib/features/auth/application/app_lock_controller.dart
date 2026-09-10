import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:local_auth/local_auth.dart';

import '../data/biometric_prefs.dart';

/// Tracks whether the app-lock screen should currently be showing. Locked
/// on cold start whenever the user has enabled biometric lock; the app
/// root re-locks it whenever the app leaves the foreground (see
/// `AppLifecycleListener` wiring in `app.dart`), so backgrounding for even
/// a moment requires another unlock — the whole point of the setting.
class AppLockController extends Notifier<bool> {
  final _auth = LocalAuthentication();

  @override
  bool build() => false;

  /// Called once bootstrap has resolved a valid session, to decide whether
  /// the lock screen should intercept it.
  Future<void> evaluateOnStart() async {
    final enabled = await ref.read(biometricPrefsProvider).isEnabled();
    state = enabled;
  }

  void lock() {
    state = true;
  }

  /// Re-locks only if the setting is on — called on every app-pause so a
  /// user who never enabled it is never interrupted.
  Future<void> lockIfEnabled() async {
    final enabled = await ref.read(biometricPrefsProvider).isEnabled();
    if (enabled) state = true;
  }

  Future<bool> tryUnlock() async {
    try {
      final ok = await _auth.authenticate(
        localizedReason: 'Unlock Adya to continue',
        options: const AuthenticationOptions(biometricOnly: false, stickyAuth: true),
      );
      if (ok) state = false;
      return ok;
    } catch (_) {
      return false;
    }
  }
}

final appLockControllerProvider = NotifierProvider<AppLockController, bool>(AppLockController.new);
