import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/messaging.dart';

class ComposerAttachment {
  ComposerAttachment({required this.fileName, required this.bytes, this.mimeType});
  final String fileName;
  final List<int> bytes;
  final String? mimeType;
}

class MessagingRepository {
  MessagingRepository(this._dio);
  final Dio _dio;

  Future<List<Channel>> listChannels() async {
    try {
      final response = await _dio.get<List<dynamic>>('/messaging/channels');
      return response.data!.map((e) => Channel.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<Channel> getChannel(String id) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/messaging/channels/$id');
      return Channel.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<Channel> createChannel({
    required String name,
    String? description,
    required bool isPrivate,
    List<String>? members,
  }) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/messaging/channels',
        data: {
          'name': name,
          'description': description,
          'is_private': isPrivate,
          'members': members,
        },
      );
      return Channel.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<ChannelMember>> listMembers(String channelId) async {
    try {
      final response = await _dio.get<List<dynamic>>('/messaging/channels/$channelId/members');
      return response.data!.map((e) => ChannelMember.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> addMember(String channelId, String userId) async {
    try {
      await _dio.post('/messaging/channels/$channelId/members', data: {'user_id': userId});
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<Message>> listMessages(String channelId) async {
    try {
      final response = await _dio.get<List<dynamic>>('/messaging/channels/$channelId/messages');
      return response.data!.map((e) => Message.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<Message> sendMessage(
    String channelId, {
    required String content,
    String? parentId,
    List<ComposerAttachment> files = const [],
  }) async {
    try {
      final formData = FormData.fromMap({
        'content': content,
        if (parentId != null) 'parentId': parentId,
        'files': [
          for (final f in files)
            MultipartFile.fromBytes(f.bytes, filename: f.fileName, contentType: null),
        ],
      });
      final response = await _dio.post<Map<String, dynamic>>(
        '/messaging/channels/$channelId/messages',
        data: formData,
      );
      return Message.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<ReactionSummary>> toggleReaction(
    String channelId,
    String messageId,
    String emoji,
  ) async {
    try {
      final response = await _dio.post<List<dynamic>>(
        '/messaging/channels/$channelId/messages/$messageId/reactions',
        data: {'emoji': emoji},
      );
      return response.data!.map((e) => ReactionSummary.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<ChannelMediaItem>> listMedia(String channelId) async {
    try {
      final response = await _dio.get<List<dynamic>>('/messaging/channels/$channelId/media');
      return response.data!.map((e) => ChannelMediaItem.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final messagingRepositoryProvider = Provider<MessagingRepository>((ref) {
  return MessagingRepository(ref.watch(dioProvider));
});
