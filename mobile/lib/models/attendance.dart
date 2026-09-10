class AttendanceRecord {
  AttendanceRecord({
    required this.id,
    required this.employeeId,
    required this.employeeName,
    required this.date,
    this.checkIn,
    this.checkOut,
    this.totalHours,
    required this.status,
    this.notes,
    this.checkInImage,
    this.checkInLat,
    this.checkInLong,
    this.checkOutLat,
    this.checkOutLong,
  });

  final String id;
  final String employeeId;
  final String employeeName;
  final DateTime date;
  final DateTime? checkIn;
  final DateTime? checkOut;
  final double? totalHours;
  final String status;
  final String? notes;
  final String? checkInImage;
  final double? checkInLat;
  final double? checkInLong;
  final double? checkOutLat;
  final double? checkOutLong;

  bool get isCheckedIn => checkIn != null && checkOut == null;
  bool get isComplete => checkIn != null && checkOut != null;

  factory AttendanceRecord.fromJson(Map<String, dynamic> json) => AttendanceRecord(
        id: json['id'] as String,
        employeeId: json['employeeId'] as String,
        employeeName: json['employeeName'] as String,
        date: DateTime.parse(json['date'] as String),
        checkIn: json['checkIn'] != null ? DateTime.parse(json['checkIn'] as String).toLocal() : null,
        checkOut:
            json['checkOut'] != null ? DateTime.parse(json['checkOut'] as String).toLocal() : null,
        totalHours: (json['totalHours'] as num?)?.toDouble(),
        status: json['status'] as String,
        notes: json['notes'] as String?,
        checkInImage: json['checkInImage'] as String?,
        checkInLat: (json['checkInLat'] as num?)?.toDouble(),
        checkInLong: (json['checkInLong'] as num?)?.toDouble(),
        checkOutLat: (json['checkOutLat'] as num?)?.toDouble(),
        checkOutLong: (json['checkOutLong'] as num?)?.toDouble(),
      );
}

class ListAttendanceResponse {
  ListAttendanceResponse({
    required this.records,
    required this.total,
  });

  final List<AttendanceRecord> records;
  final int total;

  factory ListAttendanceResponse.fromJson(Map<String, dynamic> json) => ListAttendanceResponse(
        records: (json['records'] as List<dynamic>)
            .map((e) => AttendanceRecord.fromJson(e as Map<String, dynamic>))
            .toList(),
        total: json['total'] as int,
      );
}

class MonthSummary {
  MonthSummary({
    required this.presentDays,
    required this.lateDays,
    required this.absentDays,
    required this.leaveDays,
    required this.totalHours,
  });

  final int presentDays;
  final int lateDays;
  final int absentDays;
  final double leaveDays;
  final double totalHours;

  factory MonthSummary.fromJson(Map<String, dynamic> json) => MonthSummary(
        presentDays: json['presentDays'] as int,
        lateDays: json['lateDays'] as int,
        absentDays: json['absentDays'] as int,
        leaveDays: (json['leaveDays'] as num).toDouble(),
        totalHours: (json['totalHours'] as num).toDouble(),
      );
}

class ScheduleDay {
  ScheduleDay({
    required this.dayOfWeek,
    required this.dayName,
    required this.isWorking,
    this.startTime,
    this.endTime,
  });

  final int dayOfWeek;
  final String dayName;
  final bool isWorking;
  final String? startTime;
  final String? endTime;

  factory ScheduleDay.fromJson(Map<String, dynamic> json) => ScheduleDay(
        dayOfWeek: json['dayOfWeek'] as int,
        dayName: json['dayName'] as String,
        isWorking: json['isWorking'] as bool,
        startTime: json['startTime'] as String?,
        endTime: json['endTime'] as String?,
      );
}
