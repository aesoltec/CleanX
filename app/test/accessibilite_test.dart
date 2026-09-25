// Accessibilité WCAG AA : contrastes texte ≥ 4,5:1 et libellés Semantics.
// Formule WCAG exacte (dart:math) : toute régression de palette échoue ici
// AVANT revue visuelle.
import 'dart:math' as math;

import 'package:cleanx_ui/main.dart';
import 'package:cleanx_ui/src/core/engine/moteur_simule.dart';
import 'package:cleanx_ui/src/core/providers/providers.dart';
import 'package:cleanx_ui/src/core/theme/theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

double _luminance(Color c) {
  // Formule WCAG exacte (sRGB → linéaire en puissance 2,4).
  double canal(double v) {
    return v <= 0.03928 ? v / 12.92 : math.pow((v + 0.055) / 1.055, 2.4).toDouble();
  }

  return 0.2126 * canal(c.r) + 0.7152 * canal(c.g) + 0.0722 * canal(c.b);
}

double _ratio(Color a, Color b) {
  final x = _luminance(a);
  final y = _luminance(b);
  final clair = x > y ? x : y;
  final sombre = x > y ? y : x;
  return (clair + 0.05) / (sombre + 0.05);
}

void main() {
  test('contrastes AA : paires texte/fond ≥ 4,5', () {
    const paires = {
      'blanc/vertBouton': [Colors.white, CleanXCouleurs.vertBouton],
      'vertActif/fondSombre': [
        CleanXCouleurs.vertSecurite,
        CleanXCouleurs.fondSombre
      ],
      'vertTexteClair/blanc': [CleanXCouleurs.vertTexteClair, Colors.white],
      'rougeTexteSombre/fond': [
        CleanXCouleurs.rougeTexteSombre,
        CleanXCouleurs.fondSombre
      ],
      'rougeTexteClair/blanc': [
        CleanXCouleurs.rougeTexteClair,
        Colors.white
      ],
      'rougeBouton/blanc(texte)': [Colors.white, CleanXCouleurs.rougeBouton],
      'orangeTexteClair/blanc': [
        CleanXCouleurs.orangeTexteClair,
        Colors.white
      ],
      'orange/fondSombre': [
        CleanXCouleurs.orangeAvertissement,
        CleanXCouleurs.fondSombre
      ],
      'gris/fondSombre': [CleanXCouleurs.grisTexte, CleanXCouleurs.fondSombre],
      'gris700/blanc': [
        CleanXCouleurs.texteSecondaireClair,
        Colors.white
      ],
    };
    for (final e in paires.entries) {
      final r = _ratio(e.value[0], e.value[1]);
      expect(r, greaterThanOrEqualTo(4.5), reason: e.key);
    }
  });

  testWidgets('Semantics : jauge et protection étiquetés', (t) async {
    t.view.physicalSize = const Size(390, 844);
    t.view.devicePixelRatio = 1.0;
    addTearDown(t.view.reset);
    // Sans ce handle, l'arbre de sémantique n'est pas collecté en test.
    // (dispose EXPLICITE en fin de test : la vérification de Flutter s'exécute
    // avant les addTearDown.)
    final semantics = t.ensureSemantics();
    await t.pumpWidget(
      ProviderScope(
        overrides: [moteurProvider.overrideWithValue(MoteurSimule())],
        child: const CleanXApp(),
      ),
    );
    await t.pumpAndSettle();
    // Le dashboard est bien rendu (texte jauge présent)…
    expect(find.textContaining('Score de sécurité'), findsOneWidget);
    // …et la jauge expose son libellé lecteur d'écran.
    expect(find.bySemanticsLabel(RegExp('Score de sécurité')), findsOneWidget);
    semantics.dispose();
  });
}
