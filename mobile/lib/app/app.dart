import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../core/config/app_config.dart';
import '../core/network/providers.dart';
import '../core/theme/app_theme.dart';
import '../features/auth/application/app_lock_controller.dart';
import '../features/auth/application/session_controller.dart';
import '../features/notifications/application/notification_socket.dart';
import 'router.dart';

class AdyaApp extends ConsumerStatefulWidget {
  const AdyaApp({super.key});

  @override
  ConsumerState<AdyaApp> createState() => _AdyaAppState();
}

class _AdyaAppState extends ConsumerState<AdyaApp> with WidgetsBindingObserver {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);

    // Wire the API client's session-expired callback now that the
    // provider container exists — a 401 refresh failure anywhere in the
    // app drops straight to the lock/login redirect via the router.
    Future.microtask(() {
      ref.read(sessionExpiredHandlerProvider.notifier).state =
          () => ref.read(sessionControllerProvider.notifier).forceLogout();
    });
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused || state == AppLifecycleState.detached) {
      ref.read(appLockControllerProvider.notifier).lockIfEnabled();
    }
  }

  @override
  Widget build(BuildContext context) {
    // Evaluate the app-lock once bootstrap resolves a session, and keep
    // the live notifications socket open for the life of the session.
    ref.listen(sessionControllerProvider, (previous, next) {
      if (next.value != null && previous?.value == null) {
        ref.read(appLockControllerProvider.notifier).evaluateOnStart();
      }
    });
    if (ref.watch(sessionControllerProvider).value != null) {
      ref.watch(notificationSocketProvider);
    }

    final router = ref.watch(routerProvider);

    return MaterialApp.router(
      title: AppConfig.appName,
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: ThemeMode.system,
      routerConfig: router,
    );
  }
}
