// Tests providers + moteur simulé : statut, scan (progression + fin),
// quarantaine, protection. Aucune lib native requise.
import 'package:cleanx_ui/src/core/engine/moteur.dart';
import 'package:cleanx_ui/src/core/engine/moteur_simule.dart';
import 'package:cleanx_ui/src/core/providers/providers.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

/// Conteneur Riverpod de test avec moteur simulé.
ProviderContainer _conteneur() {
  return ProviderContainer(
    overrides: [moteurProvider.overrideWithValue(MoteurSimule())],
  );
}

void main() {
  test('statut : chargement depuis le moteur', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    await c.read(statutProvider.notifier).charger();
    final statut = c.read(statutProvider).value!;
    expect(statut.signatures, 3);
    expect(statut.dossiers, isNotEmpty);
  });

  test('scan rapide : progression puis fin avec menace', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    final moteur = c.read(moteurProvider);
    await c.read(scanProvider.notifier).lancer(moteur.scanRapide);
    // Attend la fin (simule ~5 fichiers x 120 ms).
    await Future.delayed(const Duration(seconds: 2));
    final etat = c.read(scanProvider);
    expect(etat.enCours, isFalse);
    expect(etat.traites, 5);
    expect(etat.menaces, hasLength(1));
    expect(etat.menaces.first.fichier, contains('facture.pdf.exe'));
  });

  test('quarantaine : ajout puis suppression', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    final notifier = c.read(quarantaineProvider.notifier);
    await notifier.charger();
    expect(c.read(quarantaineProvider).value, isEmpty);
    await c
        .read(moteurProvider)
        .mettreEnQuarantaine(r'C:\test\malware.exe', 'Test');
    await notifier.charger();
    expect(c.read(quarantaineProvider).value, hasLength(1));
    await notifier.supprimer(1);
    expect(c.read(quarantaineProvider).value, isEmpty);
  });

  test('protection : bascule ON/OFF', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    final notifier = c.read(protectionProvider.notifier);
    await notifier.basculer(true);
    expect(c.read(protectionProvider), isTrue);
    await notifier.basculer(false);
    expect(c.read(protectionProvider), isFalse);
  });

  test('heuristique : exe suspect, txt sain', () async {
    final moteur = MoteurSimule();
    final exe = await moteur.analyserHeuristique('outil.exe');
    expect(exe.verdict, VerdictHeuristique.suspect);
    final txt = await moteur.analyserHeuristique('doc.txt');
    expect(txt.verdict, VerdictHeuristique.sain);
  });

  test('mode jeu : bascule avec état précédent', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    final notifier = c.read(modeJeuProvider.notifier);
    expect(c.read(modeJeuProvider), isFalse);
    await notifier.basculer(true);
    expect(c.read(modeJeuProvider), isTrue);
    await notifier.basculer(false);
    expect(c.read(modeJeuProvider), isFalse);
  });

  test('processus suspects : 1 suspect simulé', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    final suspects = await c.read(processusSuspectsProvider.future);
    expect(suspects, hasLength(1));
    expect(suspects.first.suspect, isTrue);
  });

  test('mode décision : Prudent par défaut, bascule explicite', () async {
    final c = _conteneur();
    addTearDown(c.dispose);
    // Défaut usine exigé par P14 (ni réseau ni disque : valeur initiale).
    expect(c.read(modeDecisionProvider), ModeDecision.prudent);
    final notifier = c.read(modeDecisionProvider.notifier);
    await notifier.definir(ModeDecision.automatique);
    expect(c.read(modeDecisionProvider), ModeDecision.automatique);
    await notifier.synchroniser();
    expect(c.read(modeDecisionProvider), ModeDecision.automatique);
  });
}
