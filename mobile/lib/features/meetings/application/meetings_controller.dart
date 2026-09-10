import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/meeting.dart';
import '../data/meetings_repository.dart';

final myMeetingsProvider = FutureProvider.autoDispose<List<Meeting>>((ref) {
  return ref.watch(meetingsRepositoryProvider).listMine();
});

final meetingDetailProvider = FutureProvider.family.autoDispose<Meeting, String>((ref, id) {
  return ref.watch(meetingsRepositoryProvider).get(id);
});

final meetingParticipantsProvider =
    FutureProvider.family.autoDispose<List<MeetingParticipant>, String>((ref, id) {
  return ref.watch(meetingsRepositoryProvider).participants(id);
});

final meetingMinutesProvider = FutureProvider.family.autoDispose<MeetingMinutes, String>((ref, id) {
  return ref.watch(meetingsRepositoryProvider).minutes(id);
});

final meetingAttachmentsProvider =
    FutureProvider.family.autoDispose<List<MeetingAttachment>, String>((ref, id) {
  return ref.watch(meetingsRepositoryProvider).attachments(id);
});
