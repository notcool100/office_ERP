import 'package:flutter/material.dart';

import '../theme/app_colors.dart';

enum ChipTone { success, warning, danger, neutral, accent }

/// The small pill labels used throughout the app for attendance/leave/
/// meeting status — "On time", "Pending", "Declined", etc.
class StatusChip extends StatelessWidget {
  const StatusChip(this.label, {super.key, this.tone = ChipTone.neutral, this.icon});

  final String label;
  final ChipTone tone;
  final IconData? icon;

  factory StatusChip.forLeaveStatus(String status) {
    switch (status.toLowerCase()) {
      case 'approved':
        return StatusChip(_titleCase(status), tone: ChipTone.success);
      case 'rejected':
      case 'declined':
        return StatusChip(_titleCase(status), tone: ChipTone.danger);
      default:
        return StatusChip(_titleCase(status), tone: ChipTone.warning);
    }
  }

  factory StatusChip.forAttendanceStatus(String status) {
    switch (status.toLowerCase()) {
      case 'present':
        return const StatusChip('On time', tone: ChipTone.success);
      case 'late':
        return const StatusChip('Late', tone: ChipTone.warning);
      case 'absent':
        return const StatusChip('Absent', tone: ChipTone.danger);
      case 'half_day':
        return const StatusChip('Half day', tone: ChipTone.neutral);
      default:
        return StatusChip(_titleCase(status), tone: ChipTone.neutral);
    }
  }

  static String _titleCase(String s) =>
      s.isEmpty ? s : '${s[0].toUpperCase()}${s.substring(1).toLowerCase()}';

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    late Color bg;
    late Color fg;
    switch (tone) {
      case ChipTone.success:
        bg = colors.successSoft;
        fg = colors.success;
        break;
      case ChipTone.warning:
        bg = colors.warningSoft;
        fg = colors.warning;
        break;
      case ChipTone.danger:
        bg = colors.dangerSoft;
        fg = colors.danger;
        break;
      case ChipTone.accent:
        bg = colors.accentSoft;
        fg = colors.accent;
        break;
      case ChipTone.neutral:
        bg = colors.surface;
        fg = colors.textMuted;
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
      decoration: BoxDecoration(
        color: bg,
        borderRadius: BorderRadius.circular(99),
        border: tone == ChipTone.neutral ? Border.all(color: colors.borderSoft) : null,
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (icon != null) ...[
            Icon(icon, size: 11, color: fg),
            const SizedBox(width: 4),
          ],
          Text(
            label,
            style: TextStyle(fontSize: 11, fontWeight: FontWeight.w700, color: fg),
          ),
        ],
      ),
    );
  }
}
