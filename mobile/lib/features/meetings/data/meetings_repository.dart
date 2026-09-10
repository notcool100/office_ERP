import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/meeting.dart';

class MeetingsRepository {
  MeetingsRepository(this._dio);
  final Dio _dio;

  Future<List<Meeting>> listMine() async {
    try {
      final response = await _dio.get<List<dynamic>>('/meetings');
      return response.data!.map((e) => Meeting.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<Meeting> get(String id) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/meetings/$id');
      return Meeting.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<MeetingParticipant>> participants(String id) async {
    try {
      final response = await _dio.get<List<dynamic>>('/meetings/$id/participants');
      return response.data!.map((e) => MeetingParticipant.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> join(String id) async {
    try {
      await _dio.post('/meetings/$id/join');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> leave(String id) async {
    try {
      await _dio.post('/meetings/$id/leave');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> end(String id) async {
    try {
      await _dio.post('/meetings/$id/end');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<MeetingMinutes> minutes(String id) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/meetings/$id/minutes');
      return MeetingMinutes.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<MeetingAttachment>> attachments(String id) async {
    try {
      final response = await _dio.get<List<dynamic>>('/meetings/$id/attachments');
      return response.data!.map((e) => MeetingAttachment.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final meetingsRepositoryProvider = Provider<MeetingsRepository>((ref) {
  return MeetingsRepository(ref.watch(dioProvider));
});
