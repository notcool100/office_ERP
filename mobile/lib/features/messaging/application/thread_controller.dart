import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/socket_stream.dart';
import '../../../models/messaging.dart';
import '../data/messaging_repository.dart';

/// Message history plus a live socket for one channel. Scoped
/// `autoDispose`, so leaving the thread screen tears the socket down
/// rather than leaving it connected in the background — the REST fetch on
/// re-entry catches it back up.
class ThreadController extends FamilyAsyncNotifier<List<Message>, String> {
  late SocketStream _socket;

  @override
  Future<List<Message>> build(String channelId) async {
    _socket = SocketStream('/ws/messaging/$channelId', requiresAuth: false);
    ref.onDispose(() => _socket.dispose());

    _socket.events.listen((event) {
      if (event.type != 'new_message') return;
      try {
        final message = Message.fromJson(event.payload as Map<String, dynamic>);
        _appendIfNew(message);
      } catch (_) {
        // Payload didn't parse as a message — a meeting-created ping on the
        // same socket, most likely. Safe to ignore here.
      }
    });
    // Connect after the listener is wired so no early frame is missed.
    unawaited(_socket.connect());

    return ref.read(messagingRepositoryProvider).listMessages(channelId);
  }

  void _appendIfNew(Message message) {
    final current = state.value ?? [];
    if (current.any((m) => m.id == message.id)) return;
    state = AsyncValue.data([...current, message]);
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(
      () => ref.read(messagingRepositoryProvider).listMessages(arg),
    );
  }

  Future<void> send({required String content, List<ComposerAttachment> files = const []}) async {
    final sent = await ref.read(messagingRepositoryProvider).sendMessage(
          arg,
          content: content,
          files: files,
        );
    _appendIfNew(sent);
  }

  Future<void> toggleReaction(String messageId, String emoji) async {
    final updated = await ref.read(messagingRepositoryProvider).toggleReaction(arg, messageId, emoji);
    final current = state.value ?? [];
    state = AsyncValue.data([
      for (final m in current) m.id == messageId ? m.copyWithReactions(updated) : m,
    ]);
  }
}

final threadControllerProvider =
    AsyncNotifierProvider.family<ThreadController, List<Message>, String>(ThreadController.new);
