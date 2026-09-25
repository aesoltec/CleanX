/// Rapport d'analyse exportable en PDF (BONUS).
///
/// Génère un document A4 : statut global, menaces détectées (session),
/// quarantaine, 50 derniers événements. Aucune dépendance native :
/// package `pdf` pur Dart. Le fichier est écrit par l'appelant (UI/tests).
library;

import 'package:pdf/pdf.dart';
import 'package:pdf/widgets.dart' as pw;

import '../engine/moteur.dart';

/// Données d'entrée du rapport (collectées par l'UI).
class DonneesRapport {
  final StatutMoteur statut;
  final List<MenaceMoteur> menacesSession;
  final List<FichierQuarantaineDto> quarantaine;
  final List<EntreeLogDto> logs;
  final DateTime date;

  const DonneesRapport({
    required this.statut,
    required this.menacesSession,
    required this.quarantaine,
    required this.logs,
    required this.date,
  });
}

/// Construit le PDF et retourne ses octets.
Future<List<int>> construireRapportPdf(DonneesRapport d) async {
  final doc = pw.Document(
    title: 'Rapport CleanX',
    author: 'CleanX Antivirus',
  );
  final vert = PdfColor.fromHex('#147A28');
  final rouge = PdfColor.fromHex('#C62828');

  doc.addPage(
    pw.MultiPage(
      pageFormat: PdfPageFormat.a4,
      header: (context) => pw.Row(
        mainAxisAlignment: pw.MainAxisAlignment.spaceBetween,
        children: [
          pw.Text('CleanX — Rapport d\'analyse',
              style: pw.TextStyle(
                  fontWeight: pw.FontWeight.bold, color: vert)),
          pw.Text(d.date.toString().substring(0, 19)),
        ],
      ),
      build: (context) => [
        pw.Header(
            level: 1,
            text: d.statut.protection
                ? 'Appareil protégé'
                : 'Protection désactivée'),
        pw.Bullet(
            text:
                'Signatures : ${d.statut.signatures} — Menaces : ${d.statut.menaces} — Fichiers analysés : ${d.statut.fichiersAnalyses}'),
        pw.Header(
            level: 2,
            text: 'Menaces de la session (${d.menacesSession.length})'),
        if (d.menacesSession.isEmpty)
          pw.Paragraph(text: 'Aucune menace détectée pendant la session.'),
        for (final m in d.menacesSession)
          pw.Bullet(
              text: '${m.fichier} — ${m.menace} [${m.action}]',
              style: pw.TextStyle(color: rouge)),
        pw.Header(
            level: 2, text: 'Quarantaine (${d.quarantaine.length})'),
        if (d.quarantaine.isEmpty)
          pw.Paragraph(text: 'Quarantaine vide.'),
        pw.TableHelper.fromTextArray(
          headers: const ['Nom', 'Date', 'Raison', 'Score'],
          data: [
            for (final f in d.quarantaine)
              [f.nom, f.date, f.raison, '${f.score}'],
          ],
        ),
        pw.Header(level: 2, text: 'Derniers événements'),
        for (final l in d.logs.take(50))
          pw.Paragraph(
              text: '[${l.date}] [${l.niveau}] ${l.message}',
              style: const pw.TextStyle(fontSize: 9)),
      ],
    ),
  );
  return doc.save();
}
