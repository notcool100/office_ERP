import 'package:intl/intl.dart';

/// Centralized date/time/number formatting so every screen renders dates
/// the same way (the web mockup mixes several formats deliberately —
/// "Sept 10", "9:42 AM", "Thu, Sept 10" — this keeps each one in one place).
class Formatters {
  Formatters._();

  static final _time = DateFormat('h:mm a');
  static final _dayMonth = DateFormat('MMM d');
  static final _weekdayDayMonth = DateFormat('EEE, MMM d');
  static final _fullDate = DateFormat('EEEE, MMMM d');
  static final _monthYear = DateFormat('MMMM yyyy');
  static final _dayNum = DateFormat('d');

  static String time(DateTime dt) => _time.format(dt);
  static String dayMonth(DateTime dt) => _dayMonth.format(dt);
  static String weekdayDayMonth(DateTime dt) => _weekdayDayMonth.format(dt);
  static String fullDate(DateTime dt) => _fullDate.format(dt);
  static String monthYear(DateTime dt) => _monthYear.format(dt);
  static String dayNum(DateTime dt) => _dayNum.format(dt);

  static String dateRange(DateTime start, DateTime end) {
    if (start.year == end.year && start.month == end.month && start.day == end.day) {
      return dayMonth(start);
    }
    return '${dayMonth(start)} – ${dayMonth(end)}';
  }

  static String relativeDay(DateTime dt) {
    final now = DateTime.now();
    final today = DateTime(now.year, now.month, now.day);
    final target = DateTime(dt.year, dt.month, dt.day);
    final diff = target.difference(today).inDays;
    if (diff == 0) return 'Today';
    if (diff == 1) return 'Tomorrow';
    if (diff == -1) return 'Yesterday';
    return weekdayDayMonth(dt);
  }

  static String relativeTime(DateTime dt) {
    final now = DateTime.now();
    final diff = now.difference(dt);
    if (diff.inSeconds < 60) return 'Just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24 && dt.day == now.day) return time(dt);
    if (diff.inDays == 1 || (diff.inHours < 48 && dt.day != now.day)) return 'Yesterday';
    if (diff.inDays < 7) return DateFormat('EEE').format(dt);
    return dayMonth(dt);
  }

  static String durationHm(double hours) {
    final h = hours.floor();
    final m = ((hours - h) * 60).round();
    if (h == 0) return '${m}m';
    return '${h}h ${m.toString().padLeft(2, '0')}m';
  }

  static String fileSize(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(0)} KB';
    return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
  }
}
