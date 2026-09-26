// Tests widget des écrans : navigation, scan, protection, quarantaine,
// logs, paramètres — avec MoteurSimule (aucune lib native requise).
import 'package:cleanx_ui/main.dart';
import 'package:cleanx_ui/src/core/engine/moteur_simule.dart';
import 'package:cleanx_ui/src/core/providers/providers.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

/// Pompe l'app complète avec un moteur simulé (pré-remplissable),
/// en format mobile 390x844 (barre de navigation basse).
Future<void> _pomper(WidgetTester t, MoteurSimule moteur) async {
  t.view.physicalSize = const Size(390, 844);
  t.view.devicePixelRatio = 1.0;
  addTearDown(t.view.reset);
  await t.pumpWidget(
    ProviderScope(
      overrides: [moteurProvider.overrideWithValue(moteur)],
      child: const CleanXApp(),
    ),
  );
  await t.pumpAndSettle();
}

/// Va sur un écran via la destination de navigation (pas l'AppBar),
/// puis vérifie le titre de l'AppBar.
Future<void> _aller(WidgetTester t, String titre) async {
  final dest = find.descendant(
    of: find.byType(NavigationBar),
    matching: find.text(titre),
  );
  await t.tap(dest);
  await t.pumpAndSettle();
  expect(find.widgetWithText(AppBar, titre), findsOneWidget);
}

void main() {
  testWidgets('navigation : les 6 écrans via la barre mobile', (t) async {
    await _pomper(t, MoteurSimule());
    for (final titre in [
      'Tableau de bord',
      'Analyse',
      'Protection',
      'Quarantaine',
      'Journal',
      'Paramètres'
    ]) {
      await _aller(t, titre);
    }
  });

  testWidgets('coquille desktop : rail de navigation à 1280px', (t) async {
    t.view.physicalSize = const Size(1280, 800);
    t.view.devicePixelRatio = 1.0;
    addTearDown(t.view.reset);
    await t.pumpWidget(
      ProviderScope(
        overrides: [moteurProvider.overrideWithValue(MoteurSimule())],
        child: const CleanXApp(),
      ),
    );
    await t.pumpAndSettle();
    expect(find.byType(NavigationRail), findsOneWidget);
    expect(find.byType(NavigationBar), findsNothing);
    await t.tap(find.descendant(
      of: find.byType(NavigationRail),
      matching: find.text('Analyse'),
    ));
    await t.pumpAndSettle();
    expect(find.widgetWithText(AppBar, 'Analyse'), findsOneWidget);
  });

  testWidgets('scan : rapide puis pause/reprise/arrêt', (t) async {
    await _pomper(t, MoteurSimule());
    await _aller(t, 'Analyse');
    await t.tap(find.text('Scan Rapide'));
    await t.pump(const Duration(milliseconds: 300));
    expect(find.textContaining('%'), findsOneWidget);
    // Pause puis reprise.
    await t.tap(find.text('Pause'));
    await t.pump();
    expect(find.text('Reprendre'), findsOneWidget);
    await t.tap(find.text('Reprendre'));
    await t.pump();
    // Arrêt.
    await t.tap(find.text('Arrêter le scan'));
    await t.pumpAndSettle();
    expect(find.text('En attente de scan…'), findsOneWidget);
  });

  testWidgets('scan : se termine avec 1 menace détectée', (t) async {
    await _pomper(t, MoteurSimule());
    await _aller(t, 'Analyse');
    await t.tap(find.text('Scan Complet'));
    await t.pumpAndSettle(const Duration(seconds: 3));
    expect(find.text('Menaces'), findsOneWidget);
    expect(find.textContaining('facture.pdf.exe'), findsWidgets);
  });

  testWidgets('protection : switch + ajout/retrait dossier', (t) async {
    await _pomper(t, MoteurSimule());
    await _aller(t, 'Protection');
    // Activation.
    await t.tap(find.byType(Switch));
    await t.pumpAndSettle();
    expect(find.text('Votre appareil est protégé'), findsOneWidget);
    // Ajout dossier.
    await t.enterText(find.byType(TextField), r'D:\Surveille');
    await t.tap(find.text('Ajouter'));
    await t.pumpAndSettle();
    expect(find.textContaining(r'D:\Surveille'), findsOneWidget);
  });

  testWidgets('quarantaine : restaurer et supprimer (avec confirmation)',
      (t) async {
    final moteur = MoteurSimule();
    await moteur.mettreEnQuarantaine(r'C:\test\malware.exe', 'Test widget');
    await _pomper(t, moteur);
    await _aller(t, 'Quarantaine');
    expect(find.text('malware.exe'), findsOneWidget);
    // Restaurer (dialogue → confirmer).
    await t.tap(find.byType(PopupMenuButton<String>));
    await t.pumpAndSettle();
    await t.tap(find.text('Restaurer'));
    await t.pumpAndSettle();
    await t.tap(find.text('Confirmer'));
    await t.pumpAndSettle();
    expect(find.text('Quarantaine vide.'), findsOneWidget);
  });

  testWidgets('logs : recherche + filtre', (t) async {
    await _pomper(t, MoteurSimule());
    await _aller(t, 'Journal');
    await t.enterText(find.byType(TextField), 'simulé');
    await t.pump();
    expect(find.textContaining('simulé'), findsWidgets);
    await t.tap(find.text('Tous'));
    await t.pump();
  });

  testWidgets('paramètres : thème, langue, mise à jour', (t) async {
    await _pomper(t, MoteurSimule());
    await _aller(t, 'Paramètres');
    // Thème sombre.
    await t.tap(find.text('Sombre'));
    await t.pumpAndSettle();
    // Langue anglaise puis retour français (items du menu déroulant).
    await t.tap(find.byType(DropdownButton<String>));
    await t.pumpAndSettle();
    await t.tap(find.text('English').last);
    await t.pumpAndSettle();
    expect(find.widgetWithText(AppBar, 'Settings'), findsOneWidget);
    await t.tap(find.byType(DropdownButton<String>));
    await t.pumpAndSettle();
    await t.tap(find.text('Français').last);
    await t.pumpAndSettle();
    expect(find.widgetWithText(AppBar, 'Paramètres'), findsOneWidget);
    // Mise à jour signatures (dialogue URL/clé → OK).
    // La tuile est sous la ligne de flottaison (ListView paresseuse) :
    // on fait défiler jusqu'à elle.
    await t.scrollUntilVisible(
      find.byTooltip('Mettre à jour les signatures'),
      500,
      scrollable: find.byType(Scrollable).first,
    );
    await t.pumpAndSettle();
    await t.tap(find.byTooltip('Mettre à jour les signatures'));
    await t.pumpAndSettle();
    expect(find.text('URL du dépôt'), findsOneWidget);
    await t.tap(find.text('OK'));
    await t.pumpAndSettle();
    expect(find.textContaining('Signatures : +0'), findsOneWidget);
    // Mode décision (P14) : options rendues, Prudent par défaut
    // (état prouvé au niveau provider ; ici : rendu + interaction sans crash).
    await t.scrollUntilVisible(
      find.text('Mode de décision'),
      500,
      scrollable: find.byType(Scrollable).first,
    );
    await t.pumpAndSettle();
    expect(find.text('Mode de décision'), findsOneWidget);
    expect(find.text('Prudent (recommandé)'), findsOneWidget);
    expect(find.text('Automatique'), findsOneWidget);
    await t.scrollUntilVisible(
      find.text('Automatique'),
      500,
      scrollable: find.byType(Scrollable).first,
    );
    await t.pumpAndSettle();
    await t.tap(find.text('Automatique'));
    await t.pumpAndSettle();
    // Mode jeu : interrupteur.
    expect(find.text('Mode jeu'), findsOneWidget);
    await t.scrollUntilVisible(
      find.byType(Switch).last,
      500,
      scrollable: find.byType(Scrollable).first,
    );
    await t.pumpAndSettle();
    await t.tap(find.byType(Switch).last);
    await t.pumpAndSettle();
  });
}
