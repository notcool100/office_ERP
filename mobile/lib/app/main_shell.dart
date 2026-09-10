import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../core/theme/app_colors.dart';
import '../features/notifications/application/notifications_provider.dart';

/// Hosts the five bottom-tab branches (Home, Calendar, Attendance, Chat,
/// More) behind one persistent `BottomNavigationBar`, matching the
/// approved design's tab bar. Everything reached from "More" (Meetings,
/// Leave, Documents, Notifications, Profile) and every detail screen push
/// full-screen above this shell — standard mobile navigation, rather than
/// keeping the tab bar visible everywhere the way the flat HTML mockup
/// did (that shortcut only worked there because it was a single page with
/// no real navigation stack).
class MainShell extends ConsumerWidget {
  const MainShell({super.key, required this.navigationShell});

  final StatefulNavigationShell navigationShell;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final unread = ref.watch(unreadNotificationCountProvider);

    return Scaffold(
      body: navigationShell,
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: navigationShell.currentIndex,
        onTap: (index) => navigationShell.goBranch(
          index,
          initialLocation: index == navigationShell.currentIndex,
        ),
        items: [
          const BottomNavigationBarItem(icon: Icon(Icons.home_rounded), label: 'Home'),
          const BottomNavigationBarItem(icon: Icon(Icons.calendar_month_rounded), label: 'Calendar'),
          const BottomNavigationBarItem(icon: Icon(Icons.access_time_rounded), label: 'Attendance'),
          const BottomNavigationBarItem(icon: Icon(Icons.chat_bubble_rounded), label: 'Chat'),
          BottomNavigationBarItem(
            icon: Badge(
              isLabelVisible: unread > 0,
              label: Text('$unread'),
              backgroundColor: colors.danger,
              child: const Icon(Icons.grid_view_rounded),
            ),
            label: 'More',
          ),
        ],
      ),
    );
  }
}
