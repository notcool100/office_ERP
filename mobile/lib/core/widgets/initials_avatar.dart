import 'package:flutter/material.dart';

import '../theme/app_colors.dart';

/// A colored circle with initials, cycling through four palette tones by
/// hashing the name — gives every person a stable, distinct-looking avatar
/// without needing a photo, matching the web app's avatar treatment.
class InitialsAvatar extends StatelessWidget {
  const InitialsAvatar(this.name, {super.key, this.size = 36, this.seed});

  final String name;
  final double size;

  /// Optional stable key (e.g. user id) to hash instead of `name`, so an
  /// avatar's color doesn't change if a display name is edited.
  final String? seed;

  String get _initials {
    final parts = name.trim().split(RegExp(r'\s+')).where((p) => p.isNotEmpty).toList();
    if (parts.isEmpty) return '?';
    if (parts.length == 1) return parts.first.substring(0, 1).toUpperCase();
    return (parts.first.substring(0, 1) + parts.last.substring(0, 1)).toUpperCase();
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final tones = [
      (colors.accentSoft, colors.accent),
      (colors.successSoft, colors.success),
      (colors.warningSoft, colors.warning),
      (colors.dangerSoft, colors.danger),
    ];
    final key = seed ?? name;
    final tone = tones[key.hashCode.abs() % tones.length];

    return Container(
      width: size,
      height: size,
      decoration: BoxDecoration(color: tone.$1, shape: BoxShape.circle),
      alignment: Alignment.center,
      child: Text(
        _initials,
        style: TextStyle(
          color: tone.$2,
          fontWeight: FontWeight.w700,
          fontSize: size * 0.36,
        ),
      ),
    );
  }
}
