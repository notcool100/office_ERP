import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:local_auth/local_auth.dart';

import '../../../core/storage/token_storage.dart';
import '../../notifications/application/push_service.dart';
import '../data/auth_repository.dart';
import '../data/session_repository.dart';
import '../../../models/user.dart';

/// Single source of truth for "who is signed in, and what do they see" —
/// null means logged out. Every screen that needs the current user reads
/// `sessionControllerProvider` rather than re-fetching bootstrap itself.
class SessionController extends AsyncNotifier<BootstrapResponse?> {
  @override
  Future<BootstrapResponse?> build() async {
    final hasSession = await TokenStorage.instance.hasSession;
    if (!hasSession) return null;
    return _loadBootstrap();
  }

  Future<BootstrapResponse?> _loadBootstrap() async {
    try {
      final bootstrap = await ref.read(sessionRepositoryProvider).bootstrap();
      // Fire-and-forget: register this device for push once we know who's
      // signed in. A failure here (no Firebase configured, permission
      // denied) must never block the user from reaching the app.
      unawaited(ref.read(pushServiceProvider).registerDevice());
      return bootstrap;
    } catch (_) {
      // Bootstrap failed even though we have a refresh token — most likely
      // the refresh token itself is dead. Treat as logged out rather than
      // showing an error screen the user can't act on.
      await TokenStorage.instance.clear();
      return null;
    }
  }

  Future<void> login({required String userName, required String password}) async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() async {
      await ref.read(authRepositoryProvider).login(userName: userName, password: password);
      return _loadBootstrap();
    });
  }

  /// Called once the caller has already proven their identity via Face
  /// ID/fingerprint — there's no separate biometric endpoint on the
  /// backend, so this simply re-loads bootstrap with the still-valid
  /// refresh token that unlocked biometrics in the first place.
  Future<void> unlockWithBiometrics() async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(_loadBootstrap);
  }

  Future<void> logout() async {
    await ref.read(pushServiceProvider).unregisterDevice();
    await ref.read(authRepositoryProvider).logout();
    state = const AsyncValue.data(null);
  }

  /// Forces the app into the logged-out state without a network call —
  /// used by [ApiClient]'s session-expired callback.
  void forceLogout() {
    state = const AsyncValue.data(null);
  }

  Future<void> refresh() async {
    final result = await AsyncValue.guard(_loadBootstrap);
    if (result.hasValue) state = result;
  }
}

final sessionControllerProvider = AsyncNotifierProvider<SessionController, BootstrapResponse?>(
  SessionController.new,
);

/// Whether Face ID/fingerprint is both available on this device and has at
/// least one credential enrolled — gates whether the login screen offers
/// the biometric shortcut at all.
final biometricAvailableProvider = FutureProvider<bool>((ref) async {
  final auth = LocalAuthentication();
  try {
    final supported = await auth.isDeviceSupported();
    final canCheck = await auth.canCheckBiometrics;
    return supported && canCheck;
  } catch (_) {
    return false;
  }
});
