import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/section_label.dart';
import '../../../core/widgets/status_chip.dart';
import '../../../features/auth/application/session_controller.dart';
import '../../../models/attendance.dart';
import '../application/attendance_controller.dart';
import 'selfie_capture_screen.dart';

class AttendanceScreen extends ConsumerStatefulWidget {
  const AttendanceScreen({super.key});

  @override
  ConsumerState<AttendanceScreen> createState() => _AttendanceScreenState();
}

class _AttendanceScreenState extends ConsumerState<AttendanceScreen> {
  late Timer _clockTimer;
  DateTime _now = DateTime.now();
  bool _submitting = false;

  @override
  void initState() {
    super.initState();
    _clockTimer = Timer.periodic(const Duration(seconds: 1), (_) {
      if (mounted) setState(() => _now = DateTime.now());
    });
  }

  @override
  void dispose() {
    _clockTimer.cancel();
    super.dispose();
  }

  Future<void> _handleCheckIn() async {
    final imageBytes = await Navigator.of(context).push<List<int>>(
      MaterialPageRoute(builder: (_) => const SelfieCaptureScreen()),
    );
    if (imageBytes == null || !mounted) return;

    setState(() => _submitting = true);
    try {
      await ref.read(attendanceControllerProvider.notifier).checkIn(imageBytes);
      ref.invalidate(attendanceHistoryProvider);
      if (mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(const SnackBar(content: Text('Checked in · face & geofence verified')));
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(_message(e))));
      }
    } finally {
      if (mounted) setState(() => _submitting = false);
    }
  }

  Future<void> _handleCheckOut() async {
    setState(() => _submitting = true);
    try {
      final record = await ref.read(attendanceControllerProvider.notifier).checkOut();
      ref.invalidate(attendanceHistoryProvider);
      if (mounted && record.totalHours != null) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Checked out · ${Formatters.durationHm(record.totalHours!)} logged')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(_message(e))));
      }
    } finally {
      if (mounted) setState(() => _submitting = false);
    }
  }

  String _message(Object e) => e is ApiException ? e.message : e.toString();

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final attendanceAsync = ref.watch(attendanceControllerProvider);
    final historyAsync = ref.watch(attendanceHistoryProvider);
    final office = ref.watch(sessionControllerProvider).value?.office;

    return Scaffold(
      appBar: AppBar(title: const Text('Attendance')),
      body: RefreshIndicator(
        onRefresh: () async {
          await ref.read(attendanceControllerProvider.notifier).refresh();
          ref.invalidate(attendanceHistoryProvider);
        },
        child: ListView(
          padding: const EdgeInsets.symmetric(horizontal: 18),
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 18),
              child: Column(
                children: [
                  Text(
                    DateFormat('h:mm:ss a').format(_now),
                    style: TextStyle(
                      fontFamily: 'monospace',
                      fontSize: 40,
                      fontWeight: FontWeight.w600,
                      color: colors.text,
                      fontFeatures: const [FontFeature.tabularFigures()],
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    Formatters.fullDate(_now),
                    style: TextStyle(fontSize: 12.5, color: colors.textMuted),
                  ),
                ],
              ),
            ),
            if (office?.latitude != null)
              Center(
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                  decoration: BoxDecoration(
                    color: colors.successSoft,
                    borderRadius: BorderRadius.circular(99),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.location_on_rounded, size: 14, color: colors.success),
                      const SizedBox(width: 6),
                      Text(
                        office?.locationName ?? office?.name ?? 'Office geofence',
                        style: TextStyle(fontSize: 12, fontWeight: FontWeight.w700, color: colors.success),
                      ),
                    ],
                  ),
                ),
              ),
            AsyncView(
              value: attendanceAsync,
              onRetry: () => ref.read(attendanceControllerProvider.notifier).refresh(),
              data: (record) => _CheckButton(
                record: record,
                submitting: _submitting,
                onCheckIn: _handleCheckIn,
                onCheckOut: _handleCheckOut,
              ),
            ),
            const SectionLabel('This month'),
            AsyncView(
              value: historyAsync,
              onRetry: () => ref.invalidate(attendanceHistoryProvider),
              data: (data) => _MonthStats(records: data.records),
            ),
            const SectionLabel('Recent history'),
            AsyncView(
              value: historyAsync,
              onRetry: () => ref.invalidate(attendanceHistoryProvider),
              data: (data) {
                final sorted = [...data.records]..sort((a, b) => b.date.compareTo(a.date));
                return AppListCard(
                  emptyLabel: 'No attendance recorded yet this month.',
                  children: sorted
                      .map(
                        (r) => AppListTile(
                          title: Formatters.weekdayDayMonth(r.date),
                          subtitle: r.checkIn != null
                              ? '${Formatters.time(r.checkIn!)} → ${r.checkOut != null ? Formatters.time(r.checkOut!) : '—'}'
                                  '${r.totalHours != null ? ' · ${Formatters.durationHm(r.totalHours!)}' : ''}'
                              : 'No check-in recorded',
                          trailing: StatusChip.forAttendanceStatus(r.status),
                        ),
                      )
                      .toList(),
                );
              },
            ),
            const SizedBox(height: 12),
          ],
        ),
      ),
    );
  }
}

class _CheckButton extends StatelessWidget {
  const _CheckButton({
    required this.record,
    required this.submitting,
    required this.onCheckIn,
    required this.onCheckOut,
  });

  final AttendanceRecord? record;
  final bool submitting;
  final VoidCallback onCheckIn;
  final VoidCallback onCheckOut;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final isCheckedIn = record?.isCheckedIn ?? false;
    final isComplete = record?.isComplete ?? false;

    final gradient = isComplete
        ? [colors.textFaint, colors.textFaint]
        : isCheckedIn
            ? [colors.danger, const Color(0xFF9E2E2E)]
            : [colors.accentHover, colors.accent];

    String label;
    String sub;
    IconData icon;
    if (isComplete) {
      label = 'Done for today';
      sub = 'See you tomorrow';
      icon = Icons.check_circle_rounded;
    } else if (isCheckedIn) {
      label = 'Check Out';
      sub = 'End your day';
      icon = Icons.camera_alt_rounded;
    } else {
      label = 'Check In';
      sub = 'Face + GPS verify';
      icon = Icons.camera_alt_rounded;
    }

    return Column(
      children: [
        GestureDetector(
          onTap: submitting || isComplete ? null : (isCheckedIn ? onCheckOut : onCheckIn),
          child: Container(
            width: 160,
            height: 160,
            margin: const EdgeInsets.symmetric(vertical: 20),
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              gradient: RadialGradient(
                center: const Alignment(-0.3, -0.4),
                colors: gradient,
              ),
            ),
            alignment: Alignment.center,
            child: submitting
                ? const CircularProgressIndicator(color: Colors.white, strokeWidth: 2.6)
                : Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(icon, color: Colors.white, size: 30),
                      const SizedBox(height: 6),
                      Text(
                        label,
                        style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w800, fontSize: 14),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        sub,
                        style: TextStyle(color: Colors.white.withValues(alpha: 0.85), fontSize: 10.5, fontWeight: FontWeight.w600),
                      ),
                    ],
                  ),
          ),
        ),
        Text(
          isCheckedIn && record?.checkIn != null
              ? 'Checked in at ${Formatters.time(record!.checkIn!)}'
              : isComplete
                  ? 'Completed today\'s attendance'
                  : 'Not checked in yet today',
          style: TextStyle(fontSize: 11.5, color: colors.textFaint),
        ),
      ],
    );
  }
}

class _MonthStats extends StatelessWidget {
  const _MonthStats({required this.records});

  final List<AttendanceRecord> records;

  @override
  Widget build(BuildContext context) {
    final present = records.where((r) => r.status == 'present').length;
    final late = records.where((r) => r.status == 'late').length;
    final absent = records.where((r) => r.status == 'absent').length;

    return Row(
      children: [
        Expanded(child: _StatTile(value: '$present', label: 'Present')),
        const SizedBox(width: 8),
        Expanded(child: _StatTile(value: '$late', label: 'Late')),
        const SizedBox(width: 8),
        Expanded(child: _StatTile(value: '$absent', label: 'Absent')),
      ],
    );
  }
}

class _StatTile extends StatelessWidget {
  const _StatTile({required this.value, required this.label});

  final String value;
  final String label;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 12),
      decoration: BoxDecoration(
        color: colors.surfaceAlt,
        border: Border.all(color: colors.borderSoft),
        borderRadius: BorderRadius.circular(14),
      ),
      child: Column(
        children: [
          Text(value, style: TextStyle(fontFamily: 'monospace', fontSize: 18, fontWeight: FontWeight.w600, color: colors.text)),
          const SizedBox(height: 2),
          Text(label.toUpperCase(), style: TextStyle(fontSize: 9.5, color: colors.textMuted, letterSpacing: 0.4)),
        ],
      ),
    );
  }
}
