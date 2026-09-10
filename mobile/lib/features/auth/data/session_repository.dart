import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/user.dart';

/// Wraps `/mobile/bootstrap` — the one call the app makes right after
/// login (and again on cold start with a valid session) to learn who the
/// caller is, what they're allowed to see, and where the office is.
class SessionRepository {
  SessionRepository(this._dio);
  final Dio _dio;

  Future<BootstrapResponse> bootstrap() async {
    try {
      final response = await _dio.get<Map<String, dynamic>>('/mobile/bootstrap');
      return BootstrapResponse.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final sessionRepositoryProvider = Provider<SessionRepository>((ref) {
  return SessionRepository(ref.watch(dioProvider));
});
