import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:table_calendar/table_calendar.dart';

import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/section_label.dart';
import '../../../models/calendar_event.dart';
import '../application/calendar_controller.dart';

class CalendarScreen extends ConsumerWidget {
  const CalendarScreen({super.key});

  Color _scopeColor(AppColors colors, String scope) {
    switch (scope) {
      case 'company':
        return colors.warning;
      case 'department':
        return colors.accent;
      default:
        return colors.success;
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final selectedDay = ref.watch(selectedDayProvider);
    final visibleMonth = ref.watch(visibleMonthProvider);
    final eventsAsync = ref.watch(monthEventsProvider(visibleMonth));

    return Scaffold(
      appBar: AppBar(title: const Text('Calendar')),
      body: RefreshIndicator(
        onRefresh: () async => ref.invalidate(monthEventsProvider(visibleMonth)),
        child: ListView(
          padding: const EdgeInsets.symmetric(horizontal: 18),
          children: [
            AsyncView(
              value: eventsAsync,
              onRetry: () => ref.invalidate(monthEventsProvider(visibleMonth)),
              data: (events) {
                final eventsByDay = <DateTime, List<CalendarEvent>>{};
                for (final e in events) {
                  final day = DateTime(e.startAt.year, e.startAt.month, e.startAt.day);
                  eventsByDay.putIfAbsent(day, () => []).add(e);
                }

                return Container(
                  margin: const EdgeInsets.only(top: 8),
                  padding: const EdgeInsets.symmetric(vertical: 6, horizontal: 4),
                  decoration: BoxDecoration(
                    color: colors.surfaceAlt,
                    border: Border.all(color: colors.borderSoft),
                    borderRadius: BorderRadius.circular(18),
                  ),
                  child: TableCalendar<CalendarEvent>(
                    firstDay: DateTime.utc(2020, 1, 1),
                    lastDay: DateTime.utc(2035, 12, 31),
                    focusedDay: visibleMonth,
                    currentDay: DateTime.now(),
                    selectedDayPredicate: (day) => isSameDay(day, selectedDay),
                    eventLoader: (day) =>
                        eventsByDay[DateTime(day.year, day.month, day.day)] ?? const [],
                    startingDayOfWeek: StartingDayOfWeek.sunday,
                    headerStyle: HeaderStyle(
                      formatButtonVisible: false,
                      titleCentered: false,
                      titleTextStyle: TextStyle(fontWeight: FontWeight.w800, fontSize: 14, color: colors.text),
                      leftChevronIcon: Icon(Icons.chevron_left_rounded, color: colors.textMuted, size: 20),
                      rightChevronIcon: Icon(Icons.chevron_right_rounded, color: colors.textMuted, size: 20),
                    ),
                    daysOfWeekStyle: DaysOfWeekStyle(
                      weekdayStyle: TextStyle(fontSize: 10, fontWeight: FontWeight.w700, color: colors.textFaint),
                      weekendStyle: TextStyle(fontSize: 10, fontWeight: FontWeight.w700, color: colors.textFaint),
                    ),
                    calendarStyle: CalendarStyle(
                      outsideDaysVisible: true,
                      defaultTextStyle: TextStyle(fontSize: 12, color: colors.text),
                      weekendTextStyle: TextStyle(fontSize: 12, color: colors.text),
                      outsideTextStyle: TextStyle(fontSize: 12, color: colors.textFaint),
                      todayDecoration: BoxDecoration(
                        shape: BoxShape.circle,
                        border: Border.all(color: colors.accent, width: 1.4),
                      ),
                      todayTextStyle: TextStyle(color: colors.accent, fontWeight: FontWeight.w700),
                      selectedDecoration: BoxDecoration(shape: BoxShape.circle, color: colors.accent),
                      selectedTextStyle: TextStyle(color: colors.accentContrast, fontWeight: FontWeight.w700),
                      markerDecoration: BoxDecoration(shape: BoxShape.circle, color: colors.accent),
                      markersMaxCount: 1,
                      markerSize: 4.5,
                      markerMargin: const EdgeInsets.only(top: 2),
                    ),
                    onDaySelected: (selected, focused) {
                      ref.read(selectedDayProvider.notifier).state =
                          DateTime(selected.year, selected.month, selected.day);
                    },
                    onPageChanged: (focused) {
                      ref.read(visibleMonthProvider.notifier).state = DateTime(focused.year, focused.month, 1);
                    },
                  ),
                );
              },
            ),
            SectionLabel(Formatters.relativeDay(selectedDay) == 'Today'
                ? '${Formatters.fullDate(selectedDay)} · Today'
                : Formatters.fullDate(selectedDay)),
            AsyncView(
              value: eventsAsync,
              onRetry: () => ref.invalidate(monthEventsProvider(visibleMonth)),
              data: (events) {
                final dayEvents = events.where((e) {
                  final day = DateTime(e.startAt.year, e.startAt.month, e.startAt.day);
                  return isSameDay(day, selectedDay);
                }).toList()
                  ..sort((a, b) => a.startAt.compareTo(b.startAt));

                if (dayEvents.isEmpty) {
                  return Padding(
                    padding: const EdgeInsets.symmetric(vertical: 30),
                    child: Center(
                      child: Text(
                        'No events scheduled this day.',
                        style: TextStyle(color: colors.textFaint, fontSize: 12.5),
                      ),
                    ),
                  );
                }

                return Column(
                  children: dayEvents
                      .map((e) => _AgendaRow(event: e, color: _scopeColor(colors, e.scope)))
                      .toList(),
                );
              },
            ),
            const SizedBox(height: 16),
          ],
        ),
      ),
    );
  }
}

class _AgendaRow extends StatelessWidget {
  const _AgendaRow({required this.event, required this.color});

  final CalendarEvent event;
  final Color color;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(width: 3, height: 38, decoration: BoxDecoration(color: color, borderRadius: BorderRadius.circular(3))),
          const SizedBox(width: 10),
          SizedBox(
            width: 54,
            child: Text(
              event.allDay ? 'All day' : Formatters.time(event.startAt),
              style: TextStyle(fontFamily: 'monospace', fontSize: 11, color: colors.textMuted),
            ),
          ),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(event.title, style: TextStyle(fontSize: 13.5, fontWeight: FontWeight.w700, color: colors.text)),
                const SizedBox(height: 2),
                Text(
                  [
                    if (!event.allDay) '${Formatters.time(event.startAt)} – ${Formatters.time(event.endAt)}',
                    _scopeLabel(event.scope),
                  ].join(' · '),
                  style: TextStyle(fontSize: 11.5, color: colors.textMuted),
                ),
                if (event.location != null && event.location!.isNotEmpty) ...[
                  const SizedBox(height: 2),
                  Row(
                    children: [
                      Icon(Icons.place_outlined, size: 12, color: colors.textFaint),
                      const SizedBox(width: 3),
                      Text(event.location!, style: TextStyle(fontSize: 11.5, color: colors.textFaint)),
                    ],
                  ),
                ],
              ],
            ),
          ),
        ],
      ),
    );
  }

  String _scopeLabel(String scope) {
    switch (scope) {
      case 'company':
        return 'Company';
      case 'department':
        return 'Department';
      default:
        return 'Personal';
    }
  }
}
