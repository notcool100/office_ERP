import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'api_client.dart';

/// Set by the app root once it can react to a forced logout (session
/// expired mid-use). Kept as a provider rather than a constructor arg so
/// [apiClientProvider] doesn't need to be recreated when the callback is
/// wired up after `ProviderScope` exists.
final sessionExpiredHandlerProvider = StateProvider<SessionExpiredCallback?>((ref) => null);

final apiClientProvider = Provider<ApiClient>((ref) {
  return ApiClient(
    onSessionExpired: () => ref.read(sessionExpiredHandlerProvider)?.call(),
  );
});

final dioProvider = Provider<Dio>((ref) => ref.watch(apiClientProvider).dio);
