/// Thème CleanX — style Kaspersky : vert sécurité, rouge alerte, orange avertissement.
/// Thème clair + sombre (Material 3), contraste renforcé (WCAG AA visé).
library;

import 'package:flutter/material.dart';

/// Palette officielle CleanX.
///
/// Contraintes WCAG AA (texte normal ≥ 4,5:1, vérifié par
/// `test/accessibilite_test.dart`) :
/// - texte blanc sur remplissage → [vertBouton] (5,46), jamais [vertSecurite]
///   (3,41 insuffisant) ;
/// - petit texte rouge → [rougeTexteSombre] en sombre (6,55), [rougeTexteClair]
///   en clair (5,62) ;
/// - petit texte vert en clair → [vertTexteClair] (6,67).
/// [vertSecurite] reste réservé aux grands éléments graphiques (jauge,
/// interrupteurs : ≥ 3:1 largement dépassés).
abstract final class CleanXCouleurs {
  static const vertSecurite = Color(0xFF21A038);
  static const vertFonce = Color(0xFF147A28);
  static const vertBouton = Color(0xFF147A28);
  static const vertTexteClair = Color(0xFF0E6B25);
  static const rougeAlerte = Color(0xFFE53935);
  static const rougeTexteSombre = Color(0xFFFF6E63);
  static const rougeTexteClair = Color(0xFFC62828);
  static const rougeBouton = Color(0xFFC62828);
  static const orangeAvertissement = Color(0xFFFB8C00);
  static const texteSecondaireClair = Color(0xFF5F6368);
  static const orangeTexteClair = Color(0xFFA84E00);
  static const fondSombre = Color(0xFF14171D);
  static const carteSombre = Color(0xFF1E232C);
  static const grisTexte = Color(0xFF9AA0AE);

  /// True si le thème actif est sombre.
  static bool estSombre(BuildContext context) =>
      Theme.of(context).brightness == Brightness.dark;

  /// Texte secondaire AA : gris clair sur sombre (6,85), gris 700 sur blanc (6,05).
  static Color texteSecondaire(BuildContext context) =>
      estSombre(context) ? grisTexte : texteSecondaireClair;

  /// Texte orange AA : orange vif sur sombre (7,57), orange brûlé sur blanc (5,59).
  static Color orangeTexte(BuildContext context) =>
      estSombre(context) ? orangeAvertissement : orangeTexteClair;

  /// Texte vert "actif" AA : vert vif sur sombre (5,26), vert profond sur blanc (6,67).
  static Color vertActif(BuildContext context) =>
      estSombre(context) ? vertSecurite : vertTexteClair;
}

class CleanXTheme {
  const CleanXTheme._();

  static ThemeData clair() {
    final base = ThemeData(useMaterial3: true, brightness: Brightness.light);
    return base.copyWith(
      colorScheme: ColorScheme.fromSeed(
        seedColor: CleanXCouleurs.vertSecurite,
        brightness: Brightness.light,
      ).copyWith(
        error: CleanXCouleurs.rougeTexteClair,
        tertiary: CleanXCouleurs.orangeAvertissement,
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: CleanXCouleurs.vertBouton,
          foregroundColor: Colors.white,
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: CleanXCouleurs.vertTexteClair,
        ),
      ),
      cardTheme: const CardThemeData(
        elevation: 1,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(16)),
        ),
      ),
    );
  }

  static ThemeData sombre() {
    final base = ThemeData(useMaterial3: true, brightness: Brightness.dark);
    return base.copyWith(
      scaffoldBackgroundColor: CleanXCouleurs.fondSombre,
      colorScheme: const ColorScheme.dark(
        primary: CleanXCouleurs.vertSecurite,
        error: CleanXCouleurs.rougeTexteSombre,
        tertiary: CleanXCouleurs.orangeAvertissement,
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: CleanXCouleurs.vertBouton,
          foregroundColor: Colors.white,
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: CleanXCouleurs.vertSecurite,
        ),
      ),
      cardTheme: const CardThemeData(
        color: CleanXCouleurs.carteSombre,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(16)),
        ),
      ),
      switchTheme: SwitchThemeData(
        thumbColor: WidgetStateProperty.resolveWith(
            (s) => s.contains(WidgetState.selected) ? Colors.white : null),
        trackColor: WidgetStateProperty.resolveWith((s) => s.contains(
                WidgetState.selected)
            ? CleanXCouleurs.vertSecurite
            : Colors.grey),
      ),
    );
  }
}
