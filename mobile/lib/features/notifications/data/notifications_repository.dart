import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/notification.dart';

class NotificationsRepository {
  NotificationsRepository(this._dio);
  final Dio _dio;

  Future<NotificationListResponse> list({DateTime? before, bool unreadOnly = false}) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>(
        '/notifications',
        queryParameters: {
          if (before != null) 'before': before.toUtc().toIso8601String(),
          'unreadOnly': unreadOnly,
        },
      );
      return NotificationListResponse.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<int> unreadCount() async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/notifications/unread-count');
      return response.data!['unreadCount'] as int;
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> markRead(String id) async {
    try {
      await _dio.post('/notifications/$id/read');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> markAllRead() async {
    try {
      await _dio.post('/notifications/read-all');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> registerDevice({
    required String token,
    required String platform,
    String? deviceName,
  }) async {
    try {
      await _dio.post(
        '/notifications/devices',
        data: {'token': token, 'platform': platform, 'deviceName': deviceName},
      );
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> unregisterDevice(String token) async {
    try {
      await _dio.delete('/notifications/devices', data: {'token': token});
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final notificationsRepositoryProvider = Provider<NotificationsRepository>((ref) {
  return NotificationsRepository(ref.watch(dioProvider));
});
