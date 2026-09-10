import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/calendar_event.dart';
import '../data/calendar_repository.dart';

/// Fetches one calendar month at a time, keyed by (year, month) — matches
/// how the screen actually consumes it (a visible month grid plus that
/// month's agenda), and means paging months doesn't refetch what's
/// already been seen this session.
final monthEventsProvider =
    FutureProvider.family.autoDispose<List<CalendarEvent>, DateTime>((ref, month) {
  final start = DateTime(month.year, month.month, 1);
  final end = DateTime(month.year, month.month + 1, 0);
  return ref.watch(calendarRepositoryProvider).listEvents(start: start, end: end);
});

final selectedDayProvider = StateProvider<DateTime>((ref) {
  final now = DateTime.now();
  return DateTime(now.year, now.month, now.day);
});

final visibleMonthProvider = StateProvider<DateTime>((ref) {
  final now = DateTime.now();
  return DateTime(now.year, now.month, 1);
});
