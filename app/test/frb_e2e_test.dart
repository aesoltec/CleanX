// Test E2E : vrai moteur Rust via FFI (lib native `cleanx_core.dll`).
//
// Prérequis : `scripts/build_windows.ps1` (construit la lib release).
// Sans la lib, le test se neutralise EXPLICITEMENT (raison documentée :
// la lib native est un artefact de build, pas une source).
// Ne crée aucun contenu malveillant : fichiers bénins + quarantaine directe.
import 'dart:io';

import 'package:cleanx_ui/src/core/engine/moteur.dart';
import 'package:cleanx_ui/src/core/engine/moteur_frb.dart';
import 'package:flutter_test/flutter_test.dart';

/// Chemin de la lib release (CWD des tests = `app/`).
String get _dll =>
    '${Directory.current.path}${Platform.pathSeparator}..${Platform.pathSeparator}core${Platform.pathSeparator}target${Platform.pathSeparator}release${Platform.pathSeparator}cleanx_core.dll';

void main() {
  test('E2E FFI : init, statut, scan, quarantaine, logs', () async {
    if (!File(_dll).existsSync()) {
      // ignore: avoid_print — diagnostic de skip volontairement visible en console CI.
      print('SKIP E2E : lib native absente ($_dll). '
          'Construisez-la : scripts/build_windows.ps1');
      return;
    }
    final MoteurCleanX moteur = MoteurFrb(cheminLib: _dll);
    final base =
        '${Directory.systemTemp.path}${Platform.pathSeparator}cleanx-e2e-${DateTime.now().millisecondsSinceEpoch}';

    // 1. Init + statut.
    final statut = await moteur.initialiser(base);
    expect(statut.signatures, greaterThanOrEqualTo(3));

    // 2. Fixtures bénins : un sain + un suspect heuristique (double extension,
    //    contenu inoffensif — aucun motif AV, aucun risque Defender).
    final fixtures = Directory('$base${Platform.pathSeparator}fixtures')
      ..createSync(recursive: true);
    File('${fixtures.path}${Platform.pathSeparator}bonjour.txt')
        .writeAsStringSync('Document de test CleanX, parfaitement inoffensif.');
    File('${fixtures.path}${Platform.pathSeparator}note.txt.exe')
        .writeAsStringSync('Binaire simulé inoffensif pour test heuristique.');
    expect(
      (await moteur.analyserHeuristique(
              '${fixtures.path}${Platform.pathSeparator}bonjour.txt'))
          .verdict,
      VerdictHeuristique.sain,
    );

    // 3. Scan personnalisé : progression + fin, sans menace.
    final evenements = await moteur.scanPersonnalise([fixtures.path]).toList();
    expect(evenements.whereType<ProgressionMoteur>(), isNotEmpty);
    final fin = evenements.whereType<ScanTermineMoteur>().single;
    expect(fin.annule, isFalse);
    expect(fin.total, 2);

    // 4. Quarantaine directe (AES-GCM réel) + restauration + suppression.
    final id = await moteur.mettreEnQuarantaine(
        '${fixtures.path}${Platform.pathSeparator}note.txt.exe',
        'Test E2E');
    expect(id, greaterThan(0));
    expect(await moteur.listerQuarantaine(), hasLength(1));
    final restaure =
        '${fixtures.path}${Platform.pathSeparator}note-restaure.exe';
    await moteur.restaurerQuarantaine(id, restaure);
    expect(File(restaure).readAsStringSync(), contains('inoffensif'));
    final id2 = await moteur.mettreEnQuarantaine(restaure, 'Test E2E (2)');
    await moteur.supprimerQuarantaine(id2);
    expect(await moteur.listerQuarantaine(), isEmpty);

    // 5. Logs persistés.
    final logs = await moteur.listerLogs(50);
    expect(logs.map((l) => l.message).join('\n'), contains('terminé'));

    // 6. Auto-protection : empreinte du binaire de test.
    expect((await moteur.integriteBinaire()).length, 64);

    // 7. Libération : le pool SQLite verrouille les fichiers le temps du
    // processus ; on libère avant suppression (le handle du log tournant
    // reste ouvert par conception tracing — suppression en best-effort).
    await moteur.libererRessources();
    try {
      Directory(base).deleteSync(recursive: true);
    } catch (_) {
      // ignore: avoid_print — diagnostic best-effort visible en console CI.
      print('NOTE E2E : dossier de base partiellement verrouillé '
          '(log tournant) — résidu temporaire acceptable.');
    }
  }, timeout: const Timeout(Duration(minutes: 3)));
}
