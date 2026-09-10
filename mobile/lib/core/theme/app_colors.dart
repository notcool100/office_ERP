import 'package:flutter/material.dart';

/// The palette from the approved Adya Mobile design (light + dark), kept as
/// plain color constants rather than baked into `ThemeData` defaults so any
/// widget can reach for `AppColors.of(context)` without threading theme
/// extensions through every constructor.
class AppColors {
  const AppColors({
    required this.bg,
    required this.surface,
    required this.surfaceAlt,
    required this.surfaceRaised,
    required this.border,
    required this.borderSoft,
    required this.text,
    required this.textMuted,
    required this.textFaint,
    required this.accent,
    required this.accentHover,
    required this.accentSoft,
    required this.accentContrast,
    required this.success,
    required this.successSoft,
    required this.warning,
    required this.warningSoft,
    required this.danger,
    required this.dangerSoft,
  });

  final Color bg;
  final Color surface;
  final Color surfaceAlt;
  final Color surfaceRaised;
  final Color border;
  final Color borderSoft;
  final Color text;
  final Color textMuted;
  final Color textFaint;
  final Color accent;
  final Color accentHover;
  final Color accentSoft;
  final Color accentContrast;
  final Color success;
  final Color successSoft;
  final Color warning;
  final Color warningSoft;
  final Color danger;
  final Color dangerSoft;

  static const light = AppColors(
    bg: Color(0xFFF5F3FC),
    surface: Color(0xFFFFFFFF),
    surfaceAlt: Color(0xFFF1EEFA),
    surfaceRaised: Color(0xFFFFFFFF),
    border: Color(0xFFE4E0F3),
    borderSoft: Color(0xFFEDEAF8),
    text: Color(0xFF1C1930),
    textMuted: Color(0xFF6E698A),
    textFaint: Color(0xFF9C97B5),
    accent: Color(0xFF5B4FE9),
    accentHover: Color(0xFF4C41D6),
    accentSoft: Color(0xFFECE9FF),
    accentContrast: Color(0xFFFFFFFF),
    success: Color(0xFF0F9668),
    successSoft: Color(0xFFE1F5EC),
    warning: Color(0xFF9A6400),
    warningSoft: Color(0xFFFBEACB),
    danger: Color(0xFFC23B3B),
    dangerSoft: Color(0xFFFBE6E6),
  );

  static const dark = AppColors(
    bg: Color(0xFF100D1C),
    surface: Color(0xFF1A1730),
    surfaceAlt: Color(0xFF221E3B),
    surfaceRaised: Color(0xFF211D3A),
    border: Color(0xFF312B52),
    borderSoft: Color(0xFF28233F),
    text: Color(0xFFEEEBFB),
    textMuted: Color(0xFFA29CC4),
    textFaint: Color(0xFF726C93),
    accent: Color(0xFF8B7EFF),
    accentHover: Color(0xFF9D92FF),
    accentSoft: Color(0xFF2A2555),
    accentContrast: Color(0xFF14102A),
    success: Color(0xFF3BD9A0),
    successSoft: Color(0xFF123328),
    warning: Color(0xFFF0B429),
    warningSoft: Color(0xFF3A2C10),
    danger: Color(0xFFFF8A8A),
    dangerSoft: Color(0xFF3B1C1E),
  );

  static AppColors of(BuildContext context) {
    return Theme.of(context).brightness == Brightness.dark ? dark : light;
  }
}
