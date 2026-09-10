import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:geolocator/geolocator.dart';

class LocationException implements Exception {
  LocationException(this.message);
  final String message;
  @override
  String toString() => message;
}

/// Wraps `geolocator`'s permission dance into one call that either returns
/// a fresh, high-accuracy fix or throws a message the check-in screen can
/// show directly.
class LocationService {
  Future<Position> getCurrentPosition() async {
    final serviceEnabled = await Geolocator.isLocationServiceEnabled();
    if (!serviceEnabled) {
      throw LocationException('Location services are turned off. Please enable them to check in.');
    }

    var permission = await Geolocator.checkPermission();
    if (permission == LocationPermission.denied) {
      permission = await Geolocator.requestPermission();
      if (permission == LocationPermission.denied) {
        throw LocationException('Location permission is required to check in.');
      }
    }

    if (permission == LocationPermission.deniedForever) {
      throw LocationException(
        'Location permission was permanently denied. Enable it from device settings.',
      );
    }

    return Geolocator.getCurrentPosition(
      locationSettings: const LocationSettings(accuracy: LocationAccuracy.high),
    );
  }

  /// Great-circle distance in meters — mirrors the backend's own haversine
  /// check, used only to show a live "within geofence" hint before the
  /// server makes the actual call.
  double distanceMeters(double lat1, double lon1, double lat2, double lon2) {
    return Geolocator.distanceBetween(lat1, lon1, lat2, lon2);
  }
}

final locationServiceProvider = Provider<LocationService>((ref) => LocationService());
