import 'dart:async';

import 'package:dio/dio.dart';
import 'package:logger/logger.dart';

import '../config/app_config.dart';
import '../storage/token_storage.dart';

/// Thrown for a 4xx/5xx response so call sites can show `message` directly
/// — every handler in the backend responds with `{"message": "..."}` or
/// `{"error": "..."}` on failure, and this normalizes both.
class ApiException implements Exception {
  ApiException(this.message, {this.statusCode});

  final String message;
  final int? statusCode;

  @override
  String toString() => message;
}

/// Raised specifically on a refresh failure, so the app can distinguish
/// "your session expired, log in again" from an ordinary request error.
class SessionExpiredException extends ApiException {
  SessionExpiredException() : super('Your session has expired. Please log in again.');
}

typedef SessionExpiredCallback = void Function();

/// Thin wrapper around Dio that owns auth headers and token refresh.
///
/// Refresh is single-flighted: if several requests 401 at once (which
/// happens constantly on a phone — the app wakes up, fires off the
/// dashboard, calendar, and notifications count in parallel, and the
/// access token happened to expire a second earlier) only the first 401
/// triggers a POST /auth/refresh; the rest wait on that same future and
/// retry with whatever token it produces. Without this, concurrent 401s
/// each refresh independently, and a slower response can land with an
/// already-superseded token and log the user out.
class ApiClient {
  // Not `this._onSessionExpired`: the field is private and the
  // constructor parameter isn't, so callers outside this library can
  // pass `onSessionExpired:` — an initializing formal would force the
  // argument label to match the private field name, which is invisible
  // outside this file.
  ApiClient({SessionExpiredCallback? onSessionExpired})
      // ignore: prefer_initializing_formals
      : _onSessionExpired = onSessionExpired {
    _dio = Dio(
      BaseOptions(
        baseUrl: AppConfig.apiBaseUrl,
        connectTimeout: const Duration(seconds: 15),
        receiveTimeout: const Duration(seconds: 30),
        sendTimeout: const Duration(seconds: 30),
        headers: {'Accept': 'application/json'},
      ),
    );

    _dio.interceptors.add(
      InterceptorsWrapper(
        onRequest: _onRequest,
        onError: _onError,
      ),
    );
  }

  late final Dio _dio;
  final SessionExpiredCallback? _onSessionExpired;
  final Logger _logger = Logger(printer: SimplePrinter());

  Completer<String?>? _refreshCompleter;

  Dio get dio => _dio;

  Future<void> _onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    final token = await TokenStorage.instance.accessToken;
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  Future<void> _onError(DioException err, ErrorInterceptorHandler handler) async {
    final response = err.response;
    final requestPath = err.requestOptions.path;

    // Never try to refresh on the auth endpoints themselves — a failed
    // login or a failed refresh is a terminal answer, not a retry signal.
    final isAuthEndpoint = requestPath.startsWith('/auth/');

    if (response?.statusCode == 401 && !isAuthEndpoint) {
      final newToken = await _refreshToken();
      if (newToken != null) {
        try {
          final retryResponse = await _retry(err.requestOptions, newToken);
          handler.resolve(retryResponse);
          return;
        } on DioException catch (retryError) {
          handler.next(retryError);
          return;
        }
      } else {
        await TokenStorage.instance.clear();
        _onSessionExpired?.call();
        handler.next(err);
        return;
      }
    }

    handler.next(err);
  }

  Future<Response<dynamic>> _retry(RequestOptions requestOptions, String token) {
    final options = Options(
      method: requestOptions.method,
      headers: {...requestOptions.headers, 'Authorization': 'Bearer $token'},
      responseType: requestOptions.responseType,
      contentType: requestOptions.contentType,
    );
    return _dio.request<dynamic>(
      requestOptions.path,
      data: requestOptions.data,
      queryParameters: requestOptions.queryParameters,
      options: options,
    );
  }

  /// Single-flight refresh: the first caller does the network round trip
  /// and stores the result in `_refreshCompleter`; anyone who arrives
  /// while that's in flight just awaits the same completer.
  Future<String?> _refreshToken() async {
    if (_refreshCompleter != null) {
      return _refreshCompleter!.future;
    }

    final completer = Completer<String?>();
    _refreshCompleter = completer;

    try {
      final refreshToken = await TokenStorage.instance.refreshToken;
      if (refreshToken == null) {
        completer.complete(null);
        return null;
      }

      // /auth/refresh is a public route — no Authorization header needed,
      // and the request interceptor would only attach the (possibly
      // already-expired) access token anyway.
      final response = await _dio.post<Map<String, dynamic>>(
        '/auth/refresh',
        data: {'refreshToken': refreshToken},
      );

      final data = response.data!;
      final accessToken = data['accessToken'] as String;
      final newRefreshToken = data['refreshToken'] as String;

      await TokenStorage.instance.saveTokens(
        accessToken: accessToken,
        refreshToken: newRefreshToken,
      );

      completer.complete(accessToken);
      return accessToken;
    } catch (e) {
      _logger.w('Token refresh failed: $e');
      completer.complete(null);
      return null;
    } finally {
      _refreshCompleter = null;
    }
  }

  /// Normalizes a DioException into an [ApiException] with the server's
  /// own message when it sent one.
  static ApiException toApiException(Object error) {
    if (error is ApiException) return error;
    if (error is DioException) {
      final response = error.response;

      // A response came back at all — this is not a connectivity problem,
      // whatever the status code says. Prefer the server's own message;
      // fall back to a message derived from the status rather than the
      // generic "check your connection" text, which is actively
      // misleading for e.g. a bodyless 401 on a bad password.
      if (response != null) {
        final data = response.data;
        String? serverMessage;
        if (data is Map) {
          serverMessage = (data['message'] ?? data['error'])?.toString();
        }
        final message = (serverMessage != null && serverMessage.isNotEmpty)
            ? serverMessage
            : _defaultMessageForStatus(response.statusCode);
        return ApiException(message, statusCode: response.statusCode);
      }

      // No response at all: this is the genuine connectivity case.
      String message = 'Could not reach the server. Please check your connection.';
      if (error.type == DioExceptionType.connectionTimeout ||
          error.type == DioExceptionType.receiveTimeout ||
          error.type == DioExceptionType.sendTimeout) {
        message = 'The server took too long to respond.';
      }
      return ApiException(message);
    }
    return ApiException(error.toString());
  }

  static String _defaultMessageForStatus(int? status) {
    switch (status) {
      case 400:
        return 'That request was not valid.';
      case 401:
        return 'Invalid credentials, or your session has expired.';
      case 403:
        return 'You do not have permission to do that.';
      case 404:
        return 'That could not be found.';
      case 409:
        return 'That conflicts with something that already exists.';
      case 422:
        return 'That request could not be processed.';
      default:
        if (status != null && status >= 500) {
          return 'The server ran into a problem. Please try again.';
        }
        return 'Something went wrong. Please try again.';
    }
  }
}
