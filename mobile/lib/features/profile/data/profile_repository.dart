import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/attendance.dart';

class ProfileRepository {
  ProfileRepository(this._dio);
  final Dio _dio;

  Future<List<ScheduleDay>> schedule() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/schedule');
      return response.data!.map((e) => ScheduleDay.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> updateProfile({String? email, String? phone}) async {
    try {
      await _dio.put('/mobile/profile', data: {'email': ?email, 'phone': ?phone});
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final profileRepositoryProvider = Provider<ProfileRepository>((ref) {
  return ProfileRepository(ref.watch(dioProvider));
});

final scheduleProvider = FutureProvider.autoDispose<List<ScheduleDay>>((ref) {
  return ref.watch(profileRepositoryProvider).schedule();
});
