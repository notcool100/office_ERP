/// Build-time configuration. Override any of these with
/// `--dart-define=KEY=value` when building for a different environment,
/// e.g.:
///
///   flutter run --dart-define=API_BASE_URL=http://10.0.2.2:3117
///
/// Defaults point at the production API so a plain `flutter run` on a
/// tester's phone talks to the real backend without extra setup.
class AppConfig {
  AppConfig._();

  static const String apiBaseUrl = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'https://api-office.adyatech.com.np',
  );

  /// Host for the `/ws/...` WebSocket endpoints — same origin as the API,
  /// just with the scheme swapped (wss for https, ws for http) so a
  /// dev override of API_BASE_URL doesn't need a second flag.
  static String get wsBaseUrl {
    final uri = Uri.parse(apiBaseUrl);
    final scheme = uri.scheme == 'https' ? 'wss' : 'ws';
    return uri.replace(scheme: scheme).toString();
  }

  static const String appName = 'Adya';

  /// Matches the server's geofence hint from `/mobile/bootstrap`, used only
  /// until that first response arrives (e.g. to size the initial map/UI
  /// affordance before bootstrap completes). The server's own check is the
  /// actual gate.
  static const double defaultGeofenceMeters = 300;
}
