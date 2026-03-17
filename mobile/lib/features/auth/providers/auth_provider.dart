import 'package:riverpod_annotation/riverpod_annotation.dart';
import '../../../core/storage/secure_storage.dart';
import '../repositories/auth_repository.dart';

part 'auth_provider.g.dart';

enum AuthState { initial, loading, authenticated, unauthenticated, error }

@riverpod
class AuthNotifier extends _$AuthNotifier {
  String? lastError;

  @override
  AuthState build() {
    _checkInitialState();
    return AuthState.initial;
  }

  Future<void> _checkInitialState() async {
    final storage = ref.read(secureStorageProvider);
    final token = await storage.getToken();
    if (token != null && token.isNotEmpty) {
      state = AuthState.authenticated;
    } else {
      state = AuthState.unauthenticated;
    }
  }

  Future<void> login(String username, String password) async {
    state = AuthState.loading;
    lastError = null;
    try {
      final repo = ref.read(authRepositoryProvider);
      final storage = ref.read(secureStorageProvider);
      
      final response = await repo.login(username, password);
      await storage.saveToken(response.accessToken);
      
      state = AuthState.authenticated;
    } catch (e) {
      print('Login error: $e');
      lastError = e.toString();
      state = AuthState.error;
    }
  }

  Future<void> logout() async {
    final storage = ref.read(secureStorageProvider);
    await storage.deleteToken();
    state = AuthState.unauthenticated;
  }
}
