import 'package:dio/dio.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/auth_response.dart';

part 'auth_repository.g.dart';

class AuthRepository {
  final Dio _client;

  AuthRepository(this._client);

  Future<AuthResponse> login(String username, String password) async {
    final response = await _client.post(
      '/auth/login',
      data: {
        'userName': username,
        'password': password,
      },
    );

    return AuthResponse.fromJson(response.data);
  }
}

@riverpod
AuthRepository authRepository(Ref ref) {
  final dio = ref.watch(apiClientProvider);
  return AuthRepository(dio);
}
