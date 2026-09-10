import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/dashboard.dart';

class DashboardRepository {
  DashboardRepository(this._dio);
  final Dio _dio;

  Future<DashboardResponse> dashboard() async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/mobile/dashboard');
      return DashboardResponse.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final dashboardRepositoryProvider = Provider<DashboardRepository>((ref) {
  return DashboardRepository(ref.watch(dioProvider));
});

final dashboardProvider = FutureProvider.autoDispose<DashboardResponse>((ref) {
  return ref.watch(dashboardRepositoryProvider).dashboard();
});
