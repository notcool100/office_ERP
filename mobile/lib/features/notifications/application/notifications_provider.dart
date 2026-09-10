import 'package:collection/collection.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/notification.dart';
import '../data/notifications_repository.dart';

class NotificationsController extends AsyncNotifier<NotificationListResponse> {
  @override
  Future<NotificationListResponse> build() {
    return ref.read(notificationsRepositoryProvider).list();
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(() => ref.read(notificationsRepositoryProvider).list());
  }

  Future<void> markRead(String id) async {
    final current = state.value;
    if (current == null) return;

    final target = current.notifications.where((n) => n.id == id).firstOrNull;
    if (target == null || !target.isUnread) return;

    // Optimistic: flip locally so the dot disappears immediately, then
    // confirm with the server. A failure just gets corrected on next pull.
    state = AsyncValue.data(
      NotificationListResponse(
        notifications: [
          for (final n in current.notifications) n.id == id ? n.copyWithRead() : n,
        ],
        unreadCount: (current.unreadCount - 1).clamp(0, 1 << 30),
      ),
    );

    try {
      await ref.read(notificationsRepositoryProvider).markRead(id);
    } catch (_) {
      await refresh();
    }
  }

  Future<void> markAllRead() async {
    final current = state.value;
    if (current == null || current.unreadCount == 0) return;

    state = AsyncValue.data(
      NotificationListResponse(
        notifications: [for (final n in current.notifications) n.copyWithRead()],
        unreadCount: 0,
      ),
    );

    try {
      await ref.read(notificationsRepositoryProvider).markAllRead();
    } catch (_) {
      await refresh();
    }
  }
}

final notificationsControllerProvider =
    AsyncNotifierProvider<NotificationsController, NotificationListResponse>(
  NotificationsController.new,
);

final unreadNotificationCountProvider = Provider<int>((ref) {
  return ref.watch(notificationsControllerProvider).value?.unreadCount ?? 0;
});
