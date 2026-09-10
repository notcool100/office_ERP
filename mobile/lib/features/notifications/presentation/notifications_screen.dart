import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/async_view.dart';
import '../../../models/notification.dart';
import '../application/notifications_provider.dart';

class NotificationsScreen extends ConsumerWidget {
  const NotificationsScreen({super.key});

  IconData _iconFor(String kind) {
    switch (kind) {
      case NotificationKind.message:
        return Icons.chat_bubble_rounded;
      case NotificationKind.meeting:
        return Icons.groups_rounded;
      case NotificationKind.leave:
        return Icons.work_history_rounded;
      case NotificationKind.calendar:
        return Icons.calendar_month_rounded;
      case NotificationKind.document:
        return Icons.folder_rounded;
      default:
        return Icons.notifications_rounded;
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final asyncNotifications = ref.watch(notificationsControllerProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Notifications'),
        actions: [
          asyncNotifications.maybeWhen(
            data: (data) => data.unreadCount > 0
                ? TextButton(
                    onPressed: () => ref.read(notificationsControllerProvider.notifier).markAllRead(),
                    child: const Text('Mark all read'),
                  )
                : const SizedBox.shrink(),
            orElse: () => const SizedBox.shrink(),
          ),
          const SizedBox(width: 4),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () => ref.read(notificationsControllerProvider.notifier).refresh(),
        child: AsyncView(
          value: asyncNotifications,
          onRetry: () => ref.invalidate(notificationsControllerProvider),
          data: (data) {
            if (data.notifications.isEmpty) {
              return ListView(
                children: const [
                  EmptyState(
                    icon: Icons.notifications_none_rounded,
                    title: 'You\'re all caught up',
                    subtitle: 'New messages, meetings and approvals will show up here.',
                  ),
                ],
              );
            }
            return ListView.builder(
              padding: const EdgeInsets.symmetric(vertical: 4),
              itemCount: data.notifications.length,
              itemBuilder: (context, index) {
                final n = data.notifications[index];
                return _NotificationRow(
                  notification: n,
                  icon: _iconFor(n.kind),
                  onTap: () => ref.read(notificationsControllerProvider.notifier).markRead(n.id),
                );
              },
            );
          },
        ),
      ),
    );
  }
}

class _NotificationRow extends StatelessWidget {
  const _NotificationRow({required this.notification, required this.icon, required this.onTap});

  final AppNotification notification;
  final IconData icon;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final unread = notification.isUnread;

    return InkWell(
      onTap: onTap,
      child: Opacity(
        opacity: unread ? 1 : 0.55,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 12),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                width: 36,
                height: 36,
                decoration: BoxDecoration(
                  color: colors.surfaceAlt,
                  borderRadius: BorderRadius.circular(11),
                  border: Border.all(color: colors.borderSoft),
                ),
                alignment: Alignment.center,
                child: Icon(icon, size: 17, color: colors.accent),
              ),
              const SizedBox(width: 11),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      notification.title,
                      style: TextStyle(fontSize: 13, fontWeight: FontWeight.w600, color: colors.text),
                    ),
                    if (notification.body != null) ...[
                      const SizedBox(height: 2),
                      Text(
                        notification.body!,
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(fontSize: 12, color: colors.textMuted),
                      ),
                    ],
                    const SizedBox(height: 4),
                    Text(
                      Formatters.relativeTime(notification.createdAt),
                      style: TextStyle(fontSize: 10.5, color: colors.textFaint, fontFeatures: const []),
                    ),
                  ],
                ),
              ),
              if (unread)
                Container(
                  width: 7,
                  height: 7,
                  margin: const EdgeInsets.only(top: 6, left: 6),
                  decoration: BoxDecoration(color: colors.accent, shape: BoxShape.circle),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
