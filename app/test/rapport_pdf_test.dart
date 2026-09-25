// Rapport PDF : génération déterministe (octets %PDF, non vide),
// avec et sans menaces.
import 'package:cleanx_ui/src/core/engine/moteur.dart';
import 'package:cleanx_ui/src/core/export/rapport_pdf.dart';
import 'package:flutter_test/flutter_test.dart';

DonneesRapport _donnees({bool avecMenace = false}) {
  return DonneesRapport(
    statut: const StatutMoteur(
      protection: true,
      signatures: 3,
      menaces: 1,
      fichiersAnalyses: 128,
      scanEnCours: false,
      dossiers: ['/tmp'],
    ),
    menacesSession: avecMenace
        ? const [
            MenaceMoteur(
              fichier: '/tmp/evil.exe',
              menace: 'Heuristique[80]',
              action: 'mis en quarantaine (#1)',
            ),
          ]
        : const [],
    quarantaine: const [
      FichierQuarantaineDto(
        id: 1,
        nom: 'evil.exe',
        origine: '/tmp/evil.exe',
        date: '2026-09-24 10:00:00',
        raison: 'Heuristique[80]',
        score: 80,
      ),
    ],
    logs: const [
      EntreeLogDto(
          id: 1,
          date: '10:00:00',
          niveau: 'quarantaine',
          message: 'evil.exe isolé'),
    ],
    date: DateTime(2026, 9, 24, 10, 0, 0),
  );
}

void main() {
  test('rapport PDF : en-tête %PDF et contenu non vide', () async {
    final octets = await construireRapportPdf(_donnees());
    expect(octets.length, greaterThan(1000));
    expect(String.fromCharCodes(octets.take(5)), '%PDF-');
  });

  test('rapport PDF : plus gros si session infectée', () async {
    final vide = await construireRapportPdf(_donnees());
    final infecte = await construireRapportPdf(_donnees(avecMenace: true));
    // Le flux étant compressé, on compare les tailles (contenu en plus).
    expect(infecte.length, greaterThan(vide.length));
  });
}
