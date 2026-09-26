/// Moteur simulé : démo locale + tests, sans lib native.
///
/// Reproduit fidèlement le protocole du vrai moteur (mêmes événements, mêmes
/// DTOs) avec des données fictives et des délais courts et déterministes.
library;

import 'moteur.dart';

/// Implémentation simulée du contrat moteur.
class MoteurSimule implements MoteurCleanX {
  bool _protection = false;
  final List<FichierQuarantaineDto> _quarantaine = [];
  final List<EntreeLogDto> _logs = [
    const EntreeLogDto(
        id: 1,
        date: '2026-09-23 10:00:00',
        niveau: 'info',
        message: 'Moteur simulé prêt'),
  ];
  int _compteurLogs = 2;

  static const _fichiersFictifs = [
    r'C:\Users\demo\Downloads\rapport.pdf',
    r'C:\Users\demo\Downloads\photo.jpg',
    r'C:\Users\demo\Desktop\notes.txt',
    r'C:\Users\demo\Downloads\outil.exe',
    r'C:\Users\demo\Downloads\facture.pdf.exe',
  ];

  void _journal(String niveau, String message) {
    _logs.insert(
        0,
        EntreeLogDto(
            id: _compteurLogs++,
            date: DateTime.now().toString().substring(0, 19),
            niveau: niveau,
            message: message));
  }

  @override
  Future<StatutMoteur> initialiser(String base) async {
    await Future.delayed(const Duration(milliseconds: 50));
    return statut();
  }

  @override
  Future<StatutMoteur> statut() async {
    return StatutMoteur(
      protection: _protection,
      signatures: 3,
      menaces: _quarantaine.length,
      fichiersAnalyses: 128,
      scanEnCours: false,
      dossiers: const [r'C:\Users\demo\Downloads', r'C:\Users\demo\Desktop'],
    );
  }

  Stream<EvenementMoteur> _scanSimule(String prefixe) async* {
    final total = _fichiersFictifs.length;
    yield JournalMoteur('Scan $prefixe : $total fichiers');
    var menaces = 0;
    for (var i = 0; i < total; i++) {
      await Future.delayed(const Duration(milliseconds: 120));
      final fichier = _fichiersFictifs[i];
      if (fichier.endsWith('facture.pdf.exe')) {
        menaces++;
        yield const MenaceMoteur(
          fichier: r'C:\Users\demo\Downloads\facture.pdf.exe',
          menace: 'Heuristique[65] : Double extension trompeuse',
          action: 'mis en quarantaine (#1)',
          score: 65,
          signaux: ['Double extension trompeuse'],
          critique: false,
          confiance: 32,
        );
      }
      yield ProgressionMoteur(
          scanId: prefixe, fichier: fichier, traites: i + 1, total: total);
    }
    yield ScanTermineMoteur(
        scanId: prefixe, total: total, menaces: menaces, annule: false);
  }

  @override
  Stream<EvenementMoteur> scanRapide() => _scanSimule('rapide-0001');

  @override
  Stream<EvenementMoteur> scanComplet() => _scanSimule('complet-0001');

  @override
  Stream<EvenementMoteur> scanPersonnalise(List<String> racines) =>
      _scanSimule('perso-0001');

  @override
  Future<bool> arreterScan() async => false;

  @override
  Future<void> suspendreScan(bool suspendre) async {}

  @override
  Stream<EvenementMoteur> activerProtection() async* {
    _protection = true;
    yield const ProtectionMoteur(true);
    yield const JournalMoteur('Protection temps réel ACTIVÉE (simulée)');
  }

  @override
  Future<bool> desactiverProtection() async {
    final active = _protection;
    _protection = false;
    return active;
  }

  @override
  Future<List<String>> dossiersSurveilles() async =>
      (await statut()).dossiers;

  @override
  Future<List<String>> ajouterDossier(String dossier) async {
    final dossiers = [...(await statut()).dossiers, dossier];
    return dossiers;
  }

  @override
  Future<List<String>> retirerDossier(String dossier) async {
    return (await statut()).dossiers.where((d) => d != dossier).toList();
  }

  @override
  Future<int> mettreEnQuarantaine(String chemin, String raison) async {
    final id = _quarantaine.length + 1;
    _quarantaine.add(FichierQuarantaineDto(
      id: id,
      nom: chemin.split(RegExp(r'[\\/]')).last,
      origine: chemin,
      date: DateTime.now().toString().substring(0, 19),
      raison: raison,
      score: 82,
    ));
    _journal('quarantaine', '$chemin isolé (#$id) : $raison');
    return id;
  }

  @override
  Future<List<FichierQuarantaineDto>> listerQuarantaine() async =>
      List.of(_quarantaine);

  @override
  Future<void> restaurerQuarantaine(int id, String destination) async {
    _quarantaine.removeWhere((f) => f.id == id);
    _journal('info', 'quarantaine #$id restaurée vers $destination');
  }

  @override
  Future<void> supprimerQuarantaine(int id) async {
    _quarantaine.removeWhere((f) => f.id == id);
    _journal('info', 'quarantaine #$id supprimée');
  }

  @override
  Future<List<EntreeLogDto>> listerLogs(int limite) async =>
      _logs.take(limite).toList();

  @override
  Future<String> exporterLogsCsv() async {
    final tampon = StringBuffer('id,date,niveau,message\n');
    for (final l in _logs) {
      tampon.writeln('${l.id},"${l.date}","${l.niveau}","${l.message}"');
    }
    return tampon.toString();
  }

  @override
  Future<int> ajouterPlanification(
      String nom, List<String> racines, int intervalleSecs) async => 1;

  @override
  Future<List<PlanificationDto>> listerPlanifications() async => const [];

  @override
  Future<void> supprimerPlanification(int id) async {}

  @override
  Future<int> mettreAJourSignatures(
          String urlDepot, String clePubliqueHex) async =>
      0;

  @override
  Future<String> integriteBinaire() async => 'simulé — lib native absente';

  @override
  Future<String> calculerSha256(String chemin) async =>
      '0' * 64; // simulation : Dart ne lit pas le disque ici

  @override
  Future<void> libererRessources() async {}

  bool _modeJeu = false;

  @override
  Future<bool> modeJeu(bool actif) async {
    final precedent = _modeJeu;
    _modeJeu = actif;
    return precedent;
  }

  @override
  Future<bool> modeJeuActif() async => _modeJeu;

  ModeDecision _modeDecision = ModeDecision.prudent;

  @override
  Future<ModeDecision> definirMode(ModeDecision mode) async {
    final precedent = _modeDecision;
    _modeDecision = mode;
    return precedent;
  }

  @override
  Future<ModeDecision> modeActuel() async => _modeDecision;

  @override
  Future<List<ProcessusDto>> listerProcessus() async => const [
        ProcessusDto(
            pid: 4, nom: 'System', memoire: 1024, signaux: []),
        ProcessusDto(
            pid: 1234,
            nom: '',
            memoire: 0,
            signaux: ['nom de processus vide (dissimulation ?)']),
      ];

  @override
  Future<List<String>> limitesRootkit() async => const [
        'Sans pilote noyau : un rootkit noyau peut masquer ce que voit userland.',
      ];

  @override
  Future<AnalyseHeuristiqueDto> analyserHeuristique(String chemin) async {
    final suspect = chemin.toLowerCase().endsWith('.exe');
    return AnalyseHeuristiqueDto(
      score: suspect ? 55 : 5,
      signaux: suspect ? const ['Extension exécutable à risque : .exe'] : const [],
      verdict: suspect ? VerdictHeuristique.suspect : VerdictHeuristique.sain,
    );
  }
}
