import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/widgets/app_list_tile.dart';
import '../../notifications/application/notifications_provider.dart';

class MoreScreen extends ConsumerWidget {
  const MoreScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final unread = ref.watch(unreadNotificationCountProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('More')),
      body: ListView(
        padding: const EdgeInsets.all(18),
        children: [
          AppListCard(
            children: [
              AppListTile(
                leading: const RowIcon(Icons.groups_rounded),
                title: 'Meetings',
                subtitle: 'Upcoming & past, minutes',
                trailing: const Icon(Icons.chevron_right_rounded, size: 20),
                onTap: () => context.push('/meetings'),
              ),
              AppListTile(
                leading: const RowIcon(Icons.work_history_rounded),
                title: 'Leave',
                subtitle: 'Balance, apply, approvals',
                trailing: const Icon(Icons.chevron_right_rounded, size: 20),
                onTap: () => context.push('/leave'),
              ),
              AppListTile(
                leading: const RowIcon(Icons.folder_rounded),
                title: 'Documents',
                subtitle: 'Company & client files',
                trailing: const Icon(Icons.chevron_right_rounded, size: 20),
                onTap: () => context.push('/documents'),
              ),
              AppListTile(
                leading: const RowIcon(Icons.notifications_rounded),
                title: 'Notifications',
                subtitle: unread > 0 ? '$unread unread' : 'All caught up',
                trailing: const Icon(Icons.chevron_right_rounded, size: 20),
                onTap: () => context.push('/notifications'),
              ),
              AppListTile(
                leading: const RowIcon(Icons.person_rounded),
                title: 'Profile',
                subtitle: 'Info & schedule',
                trailing: const Icon(Icons.chevron_right_rounded, size: 20),
                onTap: () => context.push('/profile'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
