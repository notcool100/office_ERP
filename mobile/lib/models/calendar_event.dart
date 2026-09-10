class CalendarEvent {
  CalendarEvent({
    required this.id,
    required this.title,
    this.description,
    this.location,
    required this.startAt,
    required this.endAt,
    required this.allDay,
    required this.scope,
    this.departmentId,
    this.createdBy,
  });

  final String id;
  final String title;
  final String? description;
  final String? location;
  final DateTime startAt;
  final DateTime endAt;
  final bool allDay;
  final String scope;
  final String? departmentId;
  final String? createdBy;

  factory CalendarEvent.fromJson(Map<String, dynamic> json) => CalendarEvent(
        id: json['id'] as String,
        title: json['title'] as String,
        description: json['description'] as String?,
        location: json['location'] as String?,
        startAt: DateTime.parse(json['start_at'] as String),
        endAt: DateTime.parse(json['end_at'] as String),
        allDay: json['all_day'] as bool? ?? false,
        scope: json['scope'] as String,
        departmentId: json['department_id'] as String?,
        createdBy: json['created_by'] as String?,
      );

  Map<String, dynamic> toCreateJson() => {
        'title': title,
        'description': description,
        'location': location,
        'start_at': startAt.toIso8601String(),
        'end_at': endAt.toIso8601String(),
        'all_day': allDay,
        'scope': scope,
        'department_id': departmentId,
      };
}

/// Slim event shape returned by `/mobile/dashboard` — same fields, camelCase.
class DashboardEvent {
  DashboardEvent({
    required this.id,
    required this.title,
    this.location,
    required this.startAt,
    required this.endAt,
    required this.allDay,
    required this.scope,
  });

  final String id;
  final String title;
  final String? location;
  final DateTime startAt;
  final DateTime endAt;
  final bool allDay;
  final String scope;

  factory DashboardEvent.fromJson(Map<String, dynamic> json) => DashboardEvent(
        id: json['id'] as String,
        title: json['title'] as String,
        location: json['location'] as String?,
        startAt: DateTime.parse(json['startAt'] as String),
        endAt: DateTime.parse(json['endAt'] as String),
        allDay: json['allDay'] as bool? ?? false,
        scope: json['scope'] as String,
      );
}
