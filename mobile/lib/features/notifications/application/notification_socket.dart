import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/socket_stream.dart';
import 'notifications_provider.dart';

/// Keeps the notification inbox live while the app is in the foreground,
/// independent of FCM — `/ws/notifications` is the same channel the web
/// app's bell icon uses, so a teammate approving your leave request on
/// the web shows up here within a second even on a build with no Firebase
/// project configured yet.
///
/// This is a `Provider` (not a controller) because it has no state of its
/// own — its only job is to hold the socket open and poke
/// [notificationsControllerProvider] when something arrives. Watching it
/// from the app root keeps it alive for the life of the session; disposal
/// happens automatically via `ref.onDispose` when the provider container
/// is torn down (i.e. app shutdown), since nothing ever invalidates it
/// during a session.
final notificationSocketProvider = Provider<void>((ref) {
  final socket = SocketStream('/ws/notifications');

  socket.events.listen((event) {
    if (event.type != 'notification') return;
    // The controller's own `refresh()` re-fetches from the REST endpoint
    // rather than trying to merge the raw socket payload — simpler, and
    // the payload is already the same shape `list()` returns for one row
    // so there's no data lost by not merging it directly.
    ref.read(notificationsControllerProvider.notifier).refresh();
  });

  socket.connect();
  ref.onDispose(() => socket.dispose());
});
