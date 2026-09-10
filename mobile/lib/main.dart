import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app/app.dart';
import 'features/notifications/application/push_service.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Firebase is optional at build time: a fresh checkout with no
  // `google-services.json` / `GoogleService-Info.plist` dropped in yet
  // must still run the whole app, just without push. Every push-related
  // code path checks `Firebase.apps.isNotEmpty` before doing anything, so
  // it's safe to just swallow the failure here.
  try {
    await Firebase.initializeApp();
    FirebaseMessaging.onBackgroundMessage(firebaseMessagingBackgroundHandler);
  } catch (e) {
    debugPrint('Firebase not configured for this build — push notifications disabled ($e)');
  }

  final container = ProviderContainer();
  await container.read(pushServiceProvider).initialize();

  runApp(
    UncontrolledProviderScope(
      container: container,
      child: const AdyaApp(),
    ),
  );
}
