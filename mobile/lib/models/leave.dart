class LeaveType {
  LeaveType({
    required this.id,
    required this.name,
    this.description,
    this.maxDaysPerYear,
    required this.requiresApproval,
    required this.carryForward,
  });

  final String id;
  final String name;
  final String? description;
  final int? maxDaysPerYear;
  final bool requiresApproval;
  final bool carryForward;

  factory LeaveType.fromJson(Map<String, dynamic> json) => LeaveType(
        id: json['id'] as String,
        name: json['name'] as String,
        description: json['description'] as String?,
        maxDaysPerYear: json['maxDaysPerYear'] as int?,
        requiresApproval: json['requiresApproval'] as bool? ?? true,
        carryForward: json['carryForward'] as bool? ?? false,
      );
}

class LeaveBalance {
  LeaveBalance({
    required this.employeeId,
    required this.leaveTypeId,
    required this.leaveTypeName,
    required this.totalAllowed,
    required this.used,
    required this.remaining,
  });

  final String employeeId;
  final String leaveTypeId;
  final String leaveTypeName;
  final int totalAllowed;
  final double used;
  final double remaining;

  factory LeaveBalance.fromJson(Map<String, dynamic> json) => LeaveBalance(
        employeeId: json['employeeId'] as String,
        leaveTypeId: json['leaveTypeId'] as String,
        leaveTypeName: json['leaveTypeName'] as String,
        totalAllowed: json['totalAllowed'] as int,
        used: (json['used'] as num).toDouble(),
        remaining: (json['remaining'] as num).toDouble(),
      );
}

class LeaveRequest {
  LeaveRequest({
    required this.id,
    required this.employeeId,
    required this.employeeName,
    required this.leaveTypeId,
    required this.leaveTypeName,
    required this.startDate,
    required this.endDate,
    required this.totalDays,
    this.reason,
    required this.status,
    this.approvedBy,
    this.approverName,
    this.notes,
  });

  final String id;
  final String employeeId;
  final String employeeName;
  final String leaveTypeId;
  final String leaveTypeName;
  final DateTime startDate;
  final DateTime endDate;
  final double totalDays;
  final String? reason;
  final String status;
  final String? approvedBy;
  final String? approverName;
  final String? notes;

  bool get isPending => status.toLowerCase() == 'pending';
  bool get isApproved => status.toLowerCase() == 'approved';
  bool get isRejected => status.toLowerCase() == 'rejected';

  factory LeaveRequest.fromJson(Map<String, dynamic> json) => LeaveRequest(
        id: json['id'] as String,
        employeeId: json['employeeId'] as String,
        employeeName: json['employeeName'] as String,
        leaveTypeId: json['leaveTypeId'] as String,
        leaveTypeName: json['leaveTypeName'] as String,
        startDate: DateTime.parse(json['startDate'] as String),
        endDate: DateTime.parse(json['endDate'] as String),
        totalDays: (json['totalDays'] as num).toDouble(),
        reason: json['reason'] as String?,
        status: json['status'] as String,
        approvedBy: json['approvedBy'] as String?,
        approverName: json['approverName'] as String?,
        notes: json['notes'] as String?,
      );
}
