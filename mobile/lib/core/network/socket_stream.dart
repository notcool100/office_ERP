import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../config/app_config.dart';
import '../storage/token_storage.dart';

/// One decoded frame off any of the backend's `/ws/...` endpoints — every
/// one of them emits `{"message_type": "...", "payload": {...}}` from the
/// same `WsMessage` type on the server, so a single shape covers all of
/// them.
class SocketEvent {
  SocketEvent(this.type, this.payload);
  final String type;
  final dynamic payload;

  factory SocketEvent.fromJson(Map<String, dynamic> json) =>
      SocketEvent(json['message_type'] as String? ?? '', json['payload']);
}

/// A single reconnecting WebSocket to one of the backend's live-update
/// endpoints. The backend authenticates a socket via a `token=` query
/// parameter (see `middlewares::auth::authenticate`'s WebSocket fallback),
/// so the access token has to be read fresh on every (re)connect rather
/// than passed once at construction — it can have been refreshed since.
///
/// Reconnects with capped exponential backoff whenever the socket closes
/// unexpectedly (backgrounded app, spotty network, server restart) as
/// long as [dispose] hasn't been called.
class SocketStream {
  SocketStream(this._path, {this.requiresAuth = true});

  final String _path;
  final bool requiresAuth;

  WebSocketChannel? _channel;
  StreamSubscription? _subscription;
  final _controller = StreamController<SocketEvent>.broadcast();
  bool _disposed = false;
  int _attempt = 0;

  Stream<SocketEvent> get events => _controller.stream;

  Future<void> connect() async {
    if (_disposed) return;
    await _connectOnce();
  }

  Future<void> _connectOnce() async {
    try {
      String url = '${AppConfig.wsBaseUrl}$_path';
      if (requiresAuth) {
        final token = await TokenStorage.instance.accessToken;
        if (token == null) return;
        url = '$url?token=$token';
      }

      final channel = WebSocketChannel.connect(Uri.parse(url));
      _channel = channel;
      _attempt = 0;

      _subscription = channel.stream.listen(
        (raw) {
          try {
            final decoded = jsonDecode(raw as String) as Map<String, dynamic>;
            _controller.add(SocketEvent.fromJson(decoded));
          } catch (e) {
            debugPrint('SocketStream($_path): could not decode frame: $e');
          }
        },
        onDone: _scheduleReconnect,
        onError: (_) => _scheduleReconnect(),
        cancelOnError: true,
      );
    } catch (e) {
      debugPrint('SocketStream($_path): connect failed: $e');
      _scheduleReconnect();
    }
  }

  void _scheduleReconnect() {
    if (_disposed) return;
    _attempt = (_attempt + 1).clamp(1, 6);
    final delay = Duration(seconds: 1 << _attempt); // 2s..64s
    Future.delayed(delay, () {
      if (!_disposed) _connectOnce();
    });
  }

  Future<void> dispose() async {
    _disposed = true;
    await _subscription?.cancel();
    await _channel?.sink.close();
    await _controller.close();
  }
}
