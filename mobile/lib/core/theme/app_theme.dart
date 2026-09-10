import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

import 'app_colors.dart';

/// Builds a Material `ThemeData` from an [AppColors] palette. Manrope for
/// headings/buttons (matches the web app's display font), Inter for body
/// text, JetBrains Mono for anything tabular-numeric (clock, durations,
/// employee codes) — same three-font system as the approved web mockup.
class AppTheme {
  AppTheme._();

  static ThemeData light = _build(AppColors.light, Brightness.light);
  static ThemeData dark = _build(AppColors.dark, Brightness.dark);

  static ThemeData _build(AppColors c, Brightness brightness) {
    final base = brightness == Brightness.light ? ThemeData.light() : ThemeData.dark();

    final textTheme = GoogleFonts.interTextTheme(base.textTheme).copyWith(
      displayLarge: GoogleFonts.manrope(fontWeight: FontWeight.w800, color: c.text),
      displayMedium: GoogleFonts.manrope(fontWeight: FontWeight.w800, color: c.text),
      headlineLarge: GoogleFonts.manrope(fontWeight: FontWeight.w800, color: c.text, fontSize: 28),
      headlineMedium: GoogleFonts.manrope(fontWeight: FontWeight.w800, color: c.text, fontSize: 22),
      headlineSmall: GoogleFonts.manrope(fontWeight: FontWeight.w800, color: c.text, fontSize: 18),
      titleLarge: GoogleFonts.manrope(fontWeight: FontWeight.w700, color: c.text, fontSize: 17),
      titleMedium: GoogleFonts.manrope(fontWeight: FontWeight.w700, color: c.text, fontSize: 15),
      titleSmall: GoogleFonts.manrope(fontWeight: FontWeight.w700, color: c.text, fontSize: 13),
      bodyLarge: GoogleFonts.inter(color: c.text, fontSize: 15),
      bodyMedium: GoogleFonts.inter(color: c.text, fontSize: 13.5),
      bodySmall: GoogleFonts.inter(color: c.textMuted, fontSize: 12),
      labelLarge: GoogleFonts.manrope(fontWeight: FontWeight.w700, color: c.text, fontSize: 13.5),
      labelMedium: GoogleFonts.inter(color: c.textMuted, fontSize: 11.5, fontWeight: FontWeight.w700),
      labelSmall: GoogleFonts.inter(color: c.textFaint, fontSize: 10.5, fontWeight: FontWeight.w700),
    );

    return base.copyWith(
      brightness: brightness,
      scaffoldBackgroundColor: c.bg,
      canvasColor: c.bg,
      primaryColor: c.accent,
      splashFactory: InkRipple.splashFactory,
      colorScheme: (brightness == Brightness.light
              ? const ColorScheme.light()
              : const ColorScheme.dark())
          .copyWith(
        primary: c.accent,
        onPrimary: c.accentContrast,
        secondary: c.accent,
        surface: c.surface,
        onSurface: c.text,
        error: c.danger,
        onError: Colors.white,
      ),
      textTheme: textTheme,
      appBarTheme: AppBarTheme(
        backgroundColor: c.bg,
        surfaceTintColor: Colors.transparent,
        elevation: 0,
        centerTitle: false,
        iconTheme: IconThemeData(color: c.text),
        titleTextStyle: GoogleFonts.manrope(
          fontWeight: FontWeight.w800,
          fontSize: 18,
          color: c.text,
        ),
      ),
      scrollbarTheme: const ScrollbarThemeData(thickness: WidgetStatePropertyAll(0)),
      cardTheme: CardThemeData(
        color: c.surfaceAlt,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(18),
          side: BorderSide(color: c.borderSoft),
        ),
        margin: EdgeInsets.zero,
      ),
      dividerTheme: DividerThemeData(color: c.borderSoft, thickness: 1, space: 1),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: c.accent,
          foregroundColor: c.accentContrast,
          disabledBackgroundColor: c.accent.withValues(alpha: 0.5),
          disabledForegroundColor: c.accentContrast.withValues(alpha: 0.7),
          elevation: 0,
          padding: const EdgeInsets.symmetric(vertical: 15, horizontal: 18),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(14)),
          textStyle: GoogleFonts.manrope(fontWeight: FontWeight.w700, fontSize: 14),
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          backgroundColor: c.accentSoft,
          foregroundColor: c.accent,
          side: BorderSide.none,
          elevation: 0,
          padding: const EdgeInsets.symmetric(vertical: 15, horizontal: 18),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(14)),
          textStyle: GoogleFonts.manrope(fontWeight: FontWeight.w700, fontSize: 14),
        ),
      ),
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: c.accent,
          textStyle: GoogleFonts.manrope(fontWeight: FontWeight.w700, fontSize: 14),
        ),
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: c.surfaceAlt,
        contentPadding: const EdgeInsets.symmetric(vertical: 14, horizontal: 14),
        hintStyle: GoogleFonts.inter(color: c.textFaint, fontSize: 13.5),
        labelStyle: GoogleFonts.inter(color: c.textMuted, fontSize: 12.5),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: c.border),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: c.border),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: c.accent, width: 1.4),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: c.danger),
        ),
      ),
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: c.surface,
        selectedItemColor: c.accent,
        unselectedItemColor: c.textFaint,
        type: BottomNavigationBarType.fixed,
        elevation: 0,
        selectedLabelStyle: GoogleFonts.inter(fontSize: 10.5, fontWeight: FontWeight.w700),
        unselectedLabelStyle: GoogleFonts.inter(fontSize: 10.5, fontWeight: FontWeight.w600),
      ),
      chipTheme: base.chipTheme.copyWith(
        backgroundColor: c.surfaceAlt,
        side: BorderSide(color: c.borderSoft),
        labelStyle: GoogleFonts.inter(fontSize: 11, fontWeight: FontWeight.w700, color: c.textMuted),
        padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
      ),
      dialogTheme: DialogThemeData(
        backgroundColor: c.surface,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
      ),
      bottomSheetTheme: BottomSheetThemeData(
        backgroundColor: c.surface,
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(24)),
        ),
      ),
      snackBarTheme: SnackBarThemeData(
        backgroundColor: c.text,
        contentTextStyle: GoogleFonts.inter(color: c.bg, fontSize: 12.5, fontWeight: FontWeight.w600),
        behavior: SnackBarBehavior.floating,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      ),
      progressIndicatorTheme: ProgressIndicatorThemeData(color: c.accent),
    );
  }
}
