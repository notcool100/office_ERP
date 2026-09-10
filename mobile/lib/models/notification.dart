/// Mirrors `kind::*` constants on the backend — used to pick an icon and a
/// tap destination. Any value not covered here falls back to a bell icon.
abstract class NotificationKind {
  static const message = 'message';
  static const meeting = 'meeting';
  static const leave = 'leave';
  static const calendar = 'calendar';
  static const document = 'document';
}

class AppNotification {
  AppNotification({
    required this.id,
    required this.kind,
    required this.title,
    this.body,
    this.entityType,
    this.entityId,
    this.readAt,
    required this.createdAt,
  });

  final String id;
  final String kind;
  final String title;
  final String? body;
  final String? entityType;
  final String? entityId;
  final DateTime? readAt;
  final DateTime createdAt;

  bool get isUnread => readAt == null;

  factory AppNotification.fromJson(Map<String, dynamic> json) => AppNotification(
        id: json['id'] as String,
        kind: json['kind'] as String,
        title: json['title'] as String,
        body: json['body'] as String?,
        entityType: json['entityType'] as String?,
        entityId: json['entityId'] as String?,
        readAt: json['readAt'] != null ? DateTime.parse(json['readAt'] as String).toLocal() : null,
        createdAt: DateTime.parse(json['createdAt'] as String).toLocal(),
      );

  AppNotification copyWithRead() => AppNotification(
        id: id,
        kind: kind,
        title: title,
        body: body,
        entityType: entityType,
        entityId: entityId,
        readAt: readAt ?? DateTime.now(),
        createdAt: createdAt,
      );
}

class NotificationListResponse {
  NotificationListResponse({required this.notifications, required this.unreadCount});

  final List<AppNotification> notifications;
  final int unreadCount;

  factory NotificationListResponse.fromJson(Map<String, dynamic> json) => NotificationListResponse(
        notifications: (json['notifications'] as List<dynamic>)
            .map((e) => AppNotification.fromJson(e as Map<String, dynamic>))
            .toList(),
        unreadCount: json['unreadCount'] as int,
      );
}
