import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/messaging.dart';
import '../data/messaging_repository.dart';

class ChannelsController extends AsyncNotifier<List<Channel>> {
  @override
  Future<List<Channel>> build() {
    return ref.read(messagingRepositoryProvider).listChannels();
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(() => ref.read(messagingRepositoryProvider).listChannels());
  }

  Future<Channel> createChannel({
    required String name,
    String? description,
    required bool isPrivate,
    List<String>? members,
  }) async {
    final channel = await ref.read(messagingRepositoryProvider).createChannel(
          name: name,
          description: description,
          isPrivate: isPrivate,
          members: members,
        );
    state = AsyncValue.data([channel, ...state.value ?? []]);
    return channel;
  }
}

final channelsControllerProvider = AsyncNotifierProvider<ChannelsController, List<Channel>>(
  ChannelsController.new,
);
