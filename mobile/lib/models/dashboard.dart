import 'attendance.dart';
import 'calendar_event.dart';
import 'leave.dart';
import 'meeting.dart';

class DashboardResponse {
  DashboardResponse({
    required this.date,
    required this.greetingName,
    this.attendance,
    required this.monthSummary,
    required this.events,
    required this.meetings,
    required this.pendingApprovals,
    required this.unreadNotifications,
  });

  final DateTime date;
  final String greetingName;
  final AttendanceRecord? attendance;
  final MonthSummary monthSummary;
  final List<DashboardEvent> events;
  final List<Meeting> meetings;
  final List<LeaveRequest> pendingApprovals;
  final int unreadNotifications;

  factory DashboardResponse.fromJson(Map<String, dynamic> json) => DashboardResponse(
        date: DateTime.parse(json['date'] as String),
        greetingName: json['greetingName'] as String,
        attendance:
            json['attendance'] != null ? AttendanceRecord.fromJson(json['attendance'] as Map<String, dynamic>) : null,
        monthSummary: MonthSummary.fromJson(json['monthSummary'] as Map<String, dynamic>),
        events: (json['events'] as List<dynamic>)
            .map((e) => DashboardEvent.fromJson(e as Map<String, dynamic>))
            .toList(),
        meetings: (json['meetings'] as List<dynamic>)
            .map((e) => Meeting.fromJson(e as Map<String, dynamic>))
            .toList(),
        pendingApprovals: (json['pendingApprovals'] as List<dynamic>)
            .map((e) => LeaveRequest.fromJson(e as Map<String, dynamic>))
            .toList(),
        unreadNotifications: json['unreadNotifications'] as int,
      );
}
