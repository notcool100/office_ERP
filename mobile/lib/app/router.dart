import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../features/attendance/presentation/attendance_screen.dart';
import '../features/auth/application/app_lock_controller.dart';
import '../features/auth/application/session_controller.dart';
import '../features/auth/presentation/biometric_lock_screen.dart';
import '../features/auth/presentation/login_screen.dart';
import '../features/calendar/presentation/calendar_screen.dart';
import '../features/dashboard/presentation/home_screen.dart';
import '../features/documents/presentation/documents_screen.dart';
import '../features/leave/presentation/leave_screen.dart';
import '../features/meetings/presentation/meeting_detail_screen.dart';
import '../features/meetings/presentation/meetings_list_screen.dart';
import '../features/messaging/presentation/chat_list_screen.dart';
import '../features/messaging/presentation/chat_thread_screen.dart';
import '../features/notifications/presentation/notifications_screen.dart';
import '../features/profile/presentation/more_screen.dart';
import '../features/profile/presentation/profile_screen.dart';
import '../models/messaging.dart';
import 'main_shell.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();

final routerProvider = Provider<GoRouter>((ref) {
  return GoRouter(
    navigatorKey: _rootNavigatorKey,
    initialLocation: '/home',
    refreshListenable: _RouterRefreshNotifier(ref),
    redirect: (context, state) {
      final session = ref.read(sessionControllerProvider);
      final locked = ref.read(appLockControllerProvider);
      final loggingIn = state.matchedLocation == '/login';

      // Still resolving bootstrap on cold start — hold at whatever screen
      // is showing rather than bouncing to /login and back once the
      // session turns out to be valid.
      if (session.isLoading) return null;

      final loggedIn = session.value != null;
      if (!loggedIn) return loggingIn ? null : '/login';
      if (loggedIn && loggingIn) return '/home';
      if (locked) return '/lock';
      return null;
    },
    routes: [
      GoRoute(path: '/login', builder: (context, state) => const LoginScreen()),
      GoRoute(path: '/lock', builder: (context, state) => const BiometricLockScreen()),

      StatefulShellRoute.indexedStack(
        builder: (context, state, navigationShell) => MainShell(navigationShell: navigationShell),
        branches: [
          StatefulShellBranch(routes: [
            GoRoute(path: '/home', builder: (context, state) => const HomeScreen()),
          ]),
          StatefulShellBranch(routes: [
            GoRoute(path: '/calendar', builder: (context, state) => const CalendarScreen()),
          ]),
          StatefulShellBranch(routes: [
            GoRoute(path: '/attendance', builder: (context, state) => const AttendanceScreen()),
          ]),
          StatefulShellBranch(routes: [
            GoRoute(
              path: '/chat',
              builder: (context, state) => const ChatListScreen(),
              routes: [
                GoRoute(
                  path: ':channelId',
                  parentNavigatorKey: _rootNavigatorKey,
                  builder: (context, state) => ChatThreadScreen(
                    channelId: state.pathParameters['channelId']!,
                    channel: state.extra as Channel?,
                  ),
                ),
              ],
            ),
          ]),
          StatefulShellBranch(routes: [
            GoRoute(path: '/more', builder: (context, state) => const MoreScreen()),
          ]),
        ],
      ),

      GoRoute(
        path: '/meetings',
        parentNavigatorKey: _rootNavigatorKey,
        builder: (context, state) => const MeetingsListScreen(),
        routes: [
          GoRoute(
            path: ':id',
            builder: (context, state) => MeetingDetailScreen(meetingId: state.pathParameters['id']!),
          ),
        ],
      ),
      GoRoute(
        path: '/leave',
        parentNavigatorKey: _rootNavigatorKey,
        builder: (context, state) => const LeaveScreen(),
      ),
      GoRoute(
        path: '/documents',
        parentNavigatorKey: _rootNavigatorKey,
        builder: (context, state) => const DocumentsScreen(),
      ),
      GoRoute(
        path: '/notifications',
        parentNavigatorKey: _rootNavigatorKey,
        builder: (context, state) => const NotificationsScreen(),
      ),
      GoRoute(
        path: '/profile',
        parentNavigatorKey: _rootNavigatorKey,
        builder: (context, state) => const ProfileScreen(),
      ),
    ],
  );
});

/// Bridges Riverpod state (session, app-lock) into something `GoRouter`'s
/// `refreshListenable` can watch — without this, logging in/out or
/// locking the app would change `redirect`'s inputs without ever
/// triggering a re-evaluation of them.
class _RouterRefreshNotifier extends ChangeNotifier {
  _RouterRefreshNotifier(Ref ref) {
    ref.listen(sessionControllerProvider, (_, _) => notifyListeners());
    ref.listen(appLockControllerProvider, (_, _) => notifyListeners());
  }
}
