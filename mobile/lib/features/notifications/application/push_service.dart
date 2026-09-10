import 'dart:io';

import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:package_info_plus/package_info_plus.dart';

import '../data/notifications_repository.dart';

/// Must be a top-level (or static) function — the platform calls this in
/// its own isolate when a data message arrives while the app is fully
/// killed, so it can't close over any app state.
@pragma('vm:entry-point')
Future<void> firebaseMessagingBackgroundHandler(RemoteMessage message) async {
  // Nothing to do here: a notification-type FCM message is already shown
  // by the OS itself in this state. This handler exists so data-only
  // messages have somewhere to be received without crashing; local state
  // (unread badge, inbox list) catches up next time the app opens.
}

/// Owns Firebase Cloud Messaging end to end: asking for permission,
/// registering/unregistering this device with the backend, and showing a
/// local notification when a push arrives while the app is in the
/// foreground (FCM does not surface its own banner in that state).
///
/// Every method degrades to a no-op when Firebase isn't configured for
/// this build — `google-services.json` not dropped in yet is a supported
/// state during development, not a crash.
class PushService {
  PushService(this._repository);
  final NotificationsRepository _repository;

  final _localNotifications = FlutterLocalNotificationsPlugin();
  bool _localNotificationsReady = false;
  String? _cachedToken;

  static const _androidChannel = AndroidNotificationChannel(
    'adya_default',
    'Adya notifications',
    description: 'Messages, meetings, leave and calendar updates',
    importance: Importance.high,
  );

  bool get _firebaseReady => Firebase.apps.isNotEmpty;

  Future<void> _ensureLocalNotifications() async {
    if (_localNotificationsReady) return;
    const androidInit = AndroidInitializationSettings('@mipmap/ic_launcher');
    const iosInit = DarwinInitializationSettings();
    await _localNotifications.initialize(
      const InitializationSettings(android: androidInit, iOS: iosInit),
    );
    await _localNotifications
        .resolvePlatformSpecificImplementation<AndroidFlutterLocalNotificationsPlugin>()
        ?.createNotificationChannel(_androidChannel);
    _localNotificationsReady = true;
  }

  /// Call once at app start, after `Firebase.initializeApp()` has been
  /// attempted. Wires the foreground-message listener; registration with
  /// the backend happens separately once we know who's signed in (see
  /// [registerDevice]).
  Future<void> initialize() async {
    if (!_firebaseReady) return;

    try {
      await _ensureLocalNotifications();
      FirebaseMessaging.onMessage.listen(_showForegroundNotification);
    } catch (e) {
      debugPrint('PushService.initialize failed: $e');
    }
  }

  Future<void> _showForegroundNotification(RemoteMessage message) async {
    final notification = message.notification;
    if (notification == null) return;

    await _ensureLocalNotifications();
    await _localNotifications.show(
      notification.hashCode,
      notification.title,
      notification.body,
      NotificationDetails(
        android: AndroidNotificationDetails(
          _androidChannel.id,
          _androidChannel.name,
          channelDescription: _androidChannel.description,
          importance: Importance.high,
          priority: Priority.high,
        ),
        iOS: const DarwinNotificationDetails(),
      ),
    );
  }

  /// Requests notification permission (iOS/Android 13+ prompt) and sends
  /// the FCM token to the backend. Safe to call every time the session
  /// changes — the backend upserts on the token, so re-registering the
  /// same device is a no-op.
  Future<void> registerDevice() async {
    if (!_firebaseReady) return;

    try {
      final messaging = FirebaseMessaging.instance;
      final settings = await messaging.requestPermission(alert: true, badge: true, sound: true);
      if (settings.authorizationStatus == AuthorizationStatus.denied) return;

      final token = await messaging.getToken();
      if (token == null) return;
      _cachedToken = token;

      final info = await PackageInfo.fromPlatform();
      await _repository.registerDevice(
        token: token,
        platform: Platform.isIOS ? 'ios' : 'android',
        deviceName: '${info.appName} ${info.version}',
      );

      // A token can rotate at any time (app reinstall, backup restore) —
      // re-register transparently so the old one doesn't silently go stale.
      FirebaseMessaging.instance.onTokenRefresh.listen((newToken) {
        _cachedToken = newToken;
        _repository.registerDevice(
          token: newToken,
          platform: Platform.isIOS ? 'ios' : 'android',
        );
      });
    } catch (e) {
      debugPrint('PushService.registerDevice failed: $e');
    }
  }

  /// Called on logout so a shared/handed-down device stops receiving the
  /// previous account's pushes.
  Future<void> unregisterDevice() async {
    if (!_firebaseReady || _cachedToken == null) return;
    try {
      await _repository.unregisterDevice(_cachedToken!);
    } catch (e) {
      debugPrint('PushService.unregisterDevice failed: $e');
    }
  }
}

final pushServiceProvider = Provider<PushService>((ref) {
  return PushService(ref.watch(notificationsRepositoryProvider));
});
