class MobileUser {
  MobileUser({
    required this.id,
    required this.userName,
    required this.email,
    required this.phone,
    required this.isAdmin,
  });

  final String id;
  final String userName;
  final String email;
  final String phone;
  final bool isAdmin;

  factory MobileUser.fromJson(Map<String, dynamic> json) => MobileUser(
        id: json['id'] as String,
        userName: json['userName'] as String,
        email: json['email'] as String,
        phone: json['phone'] as String,
        isAdmin: json['isAdmin'] as bool? ?? false,
      );
}

class MobilePerson {
  MobilePerson({
    required this.id,
    required this.firstName,
    this.middleName,
    required this.lastName,
    required this.fullName,
    required this.initials,
    this.email,
    this.phone,
  });

  final String id;
  final String firstName;
  final String? middleName;
  final String lastName;
  final String fullName;
  final String initials;
  final String? email;
  final String? phone;

  factory MobilePerson.fromJson(Map<String, dynamic> json) => MobilePerson(
        id: json['id'] as String,
        firstName: json['firstName'] as String,
        middleName: json['middleName'] as String?,
        lastName: json['lastName'] as String,
        fullName: json['fullName'] as String,
        initials: json['initials'] as String,
        email: json['email'] as String?,
        phone: json['phone'] as String?,
      );
}

class MobileEmployee {
  MobileEmployee({
    required this.id,
    required this.employeeCode,
    this.department,
    this.position,
    required this.hireDate,
    this.employmentType,
    required this.status,
    this.managerId,
    this.managerName,
  });

  final String id;
  final String employeeCode;
  final String? department;
  final String? position;
  final DateTime hireDate;
  final String? employmentType;
  final String status;
  final String? managerId;
  final String? managerName;

  factory MobileEmployee.fromJson(Map<String, dynamic> json) => MobileEmployee(
        id: json['id'] as String,
        employeeCode: json['employeeCode'] as String,
        department: json['department'] as String?,
        position: json['position'] as String?,
        hireDate: DateTime.parse(json['hireDate'] as String),
        employmentType: json['employmentType'] as String?,
        status: json['status'] as String,
        managerId: json['managerId'] as String?,
        managerName: json['managerName'] as String?,
      );
}

class MobileCapabilities {
  MobileCapabilities({
    required this.companyDocuments,
    required this.clientDocuments,
    required this.leaveApprovals,
  });

  final bool companyDocuments;
  final bool clientDocuments;
  final bool leaveApprovals;

  factory MobileCapabilities.fromJson(Map<String, dynamic> json) => MobileCapabilities(
        companyDocuments: json['companyDocuments'] as bool? ?? false,
        clientDocuments: json['clientDocuments'] as bool? ?? false,
        leaveApprovals: json['leaveApprovals'] as bool? ?? false,
      );
}

class MobileOffice {
  MobileOffice({
    required this.name,
    this.latitude,
    this.longitude,
    this.locationName,
    required this.geofenceMeters,
  });

  final String name;
  final double? latitude;
  final double? longitude;
  final String? locationName;
  final double geofenceMeters;

  factory MobileOffice.fromJson(Map<String, dynamic> json) => MobileOffice(
        name: json['name'] as String,
        latitude: (json['latitude'] as num?)?.toDouble(),
        longitude: (json['longitude'] as num?)?.toDouble(),
        locationName: json['locationName'] as String?,
        geofenceMeters: (json['geofenceMeters'] as num?)?.toDouble() ?? 300,
      );
}

class BootstrapResponse {
  BootstrapResponse({
    required this.user,
    required this.person,
    this.employee,
    required this.capabilities,
    required this.office,
  });

  final MobileUser user;
  final MobilePerson person;
  final MobileEmployee? employee;
  final MobileCapabilities capabilities;
  final MobileOffice office;

  factory BootstrapResponse.fromJson(Map<String, dynamic> json) => BootstrapResponse(
        user: MobileUser.fromJson(json['user'] as Map<String, dynamic>),
        person: MobilePerson.fromJson(json['person'] as Map<String, dynamic>),
        employee: json['employee'] != null
            ? MobileEmployee.fromJson(json['employee'] as Map<String, dynamic>)
            : null,
        capabilities: MobileCapabilities.fromJson(json['capabilities'] as Map<String, dynamic>),
        office: MobileOffice.fromJson(json['office'] as Map<String, dynamic>),
      );
}
