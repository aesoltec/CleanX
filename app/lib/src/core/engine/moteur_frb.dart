/// Pont FFI réel vers le moteur Rust (`cleanx_core`).
///
/// Convertit les types générés (`lib/src/rust/`) vers les DTOs du contrat
/// [MoteurCleanX]. Nécessite la lib native (voir `scripts/build_windows.ps1`)
/// et l'initialisation `CleanxCore.init()` (appelée ici, une seule fois).
/// Actif avec `--dart-define=CLEANX_FRB=true`.
library;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart'
    show ExternalLibrary;
import 'package:path_provider/path_provider.dart';

import '../../rust/api.dart' as noyau;
import '../../rust/frb_generated.dart';
import '../../rust/heuristics.dart' as noyau_h;
import '../../rust/logging.dart' as noyau_l;
import '../../rust/mode.dart' as noyau_m;
import '../../rust/quarantine.dart' as noyau_q;
import '../../rust/rootkit.dart' as noyau_r;
import '../../rust/scheduler.dart' as noyau_s;
import 'moteur.dart';

/// Implémentation FFI du contrat moteur.
class MoteurFrb implements MoteurCleanX {
  /// Chemin explicite vers la lib native (tests E2E). `null` = chargement
  /// par défaut (dossier de l'exécutable, cf. `scripts/build_windows.ps1`).
  final String? cheminLib;
  MoteurFrb({this.cheminLib});

  bool _initFfi = false;

  /// Charge la lib native (idempotent).
  Future<void> _ffi() async {
    if (!_initFfi) {
      if (cheminLib != null) {
        await CleanxCore.init(
            externalLibrary: ExternalLibrary.open(cheminLib!));
      } else {
        await CleanxCore.init();
      }
      _initFfi = true;
    }
  }

  @override
  Future<StatutMoteur> initialiser(String base) async {
    await _ffi();
    if (base.isEmpty) {
      base = (await getApplicationSupportDirectory()).path;
    }
    return _statut(noyau.initialiser(base: base));
  }

  @override
  Future<StatutMoteur> statut() async {
    await _ffi();
    return _statut(noyau.statut());
  }

  StatutMoteur _statut(noyau.StatutGlobal s) => StatutMoteur(
        protection: s.protection,
        signatures: s.signatures,
        menaces: s.menaces.toInt(),
        fichiersAnalyses: s.fichiersAnalyses.toInt(),
        scanEnCours: s.scanEnCours,
        dossiers: s.dossiers,
      );

  Stream<EvenementMoteur> _convertir(
      Stream<noyau.EvenementMoteur> source) {
    return source.map((e) {
      if (e is noyau.EvenementMoteur_Journal) {
        return JournalMoteur(e.message);
      } else if (e is noyau.EvenementMoteur_Progression) {
        return ProgressionMoteur(
          scanId: e.scanId,
          fichier: e.fichier,
          traites: e.traites.toInt(),
          total: e.total.toInt(),
        );
      } else if (e is noyau.EvenementMoteur_Menace) {
        return MenaceMoteur(
          fichier: e.fichier,
          menace: e.menace,
          action: e.action,
          sha256: e.sha256,
          score: e.score,
          signaux: e.signaux,
          critique: e.critique,
          confiance: e.confiance,
        );
      } else if (e is noyau.EvenementMoteur_ScanTermine) {
        return ScanTermineMoteur(
          scanId: e.scanId,
          total: e.total.toInt(),
          menaces: e.menaces.toInt(),
          annule: e.annule,
        );
      } else if (e is noyau.EvenementMoteur_Protection) {
        return ProtectionMoteur(e.active);
      }
      throw StateError('Événement moteur inconnu : $e');
    });
  }

  @override
  Stream<EvenementMoteur> scanRapide() => _convertir(noyau.scanRapide());

  @override
  Stream<EvenementMoteur> scanComplet() => _convertir(noyau.scanComplet());

  @override
  Stream<EvenementMoteur> scanPersonnalise(List<String> racines) =>
      _convertir(noyau.scanPersonnalise(racines: racines));

  @override
  Future<bool> arreterScan() async {
    await _ffi();
    return noyau.arreterScan();
  }

  @override
  Future<void> suspendreScan(bool suspendre) async {
    await _ffi();
    noyau.suspendreScan(suspendre: suspendre);
  }

  @override
  Stream<EvenementMoteur> activerProtection() =>
      _convertir(noyau.activerProtection());

  @override
  Future<bool> desactiverProtection() async {
    await _ffi();
    return noyau.desactiverProtection();
  }

  @override
  Future<List<String>> dossiersSurveilles() async {
    await _ffi();
    return noyau.dossiersSurveilles();
  }

  @override
  Future<List<String>> ajouterDossier(String dossier) async {
    await _ffi();
    return noyau.ajouterDossier(dossier: dossier);
  }

  @override
  Future<List<String>> retirerDossier(String dossier) async {
    await _ffi();
    return noyau.retirerDossier(dossier: dossier);
  }

  @override
  Future<int> mettreEnQuarantaine(String chemin, String raison) async {
    await _ffi();
    return noyau.mettreEnQuarantaine(chemin: chemin, raison: raison).toInt();
  }

  @override
  Future<List<FichierQuarantaineDto>> listerQuarantaine() async {
    await _ffi();
    return noyau.listerQuarantaine().map(_quarantaine).toList();
  }

  FichierQuarantaineDto _quarantaine(noyau_q.FichierQuarantaine f) =>
      FichierQuarantaineDto(
        id: f.id.toInt(),
        nom: f.nom,
        origine: f.origine,
        date: f.date,
        raison: f.raison,
        score: f.score,
      );

  @override
  Future<void> restaurerQuarantaine(int id, String destination) async {
    await _ffi();
    noyau.restaurerQuarantaine(id: id, destination: destination);
  }

  @override
  Future<void> supprimerQuarantaine(int id) async {
    await _ffi();
    noyau.supprimerQuarantaine(id: id);
  }

  @override
  Future<List<EntreeLogDto>> listerLogs(int limite) async {
    await _ffi();
    return noyau.listerLogs(limite: limite).map(_log).toList();
  }

  EntreeLogDto _log(noyau_l.EntreeLog l) => EntreeLogDto(
        id: l.id.toInt(),
        date: l.date,
        niveau: l.niveau,
        message: l.message,
      );

  @override
  Future<String> exporterLogsCsv() async {
    await _ffi();
    return noyau.exporterLogsCsv();
  }

  @override
  Future<int> ajouterPlanification(
      String nom, List<String> racines, int intervalleSecs) async {
    await _ffi();
    return noyau
        .ajouterPlanification(
          nom: nom,
          racines: racines,
          intervalleSecs: BigInt.from(intervalleSecs),
        )
        .toInt();
  }

  @override
  Future<List<PlanificationDto>> listerPlanifications() async {
    await _ffi();
    return noyau.listerPlanifications().map(_planification).toList();
  }

  PlanificationDto _planification(noyau_s.Planification p) =>
      PlanificationDto(
        id: p.id.toInt(),
        nom: p.nom,
        racines: p.racines,
        intervalleSecs: p.intervalleSecs.toInt(),
        prochaineExec: p.prochaineExec.toInt(),
      );

  @override
  Future<void> supprimerPlanification(int id) async {
    await _ffi();
    noyau.supprimerPlanification(id: id);
  }

  @override
  Future<int> mettreAJourSignatures(
      String urlDepot, String clePubliqueHex) async {
    await _ffi();
    return noyau.mettreAJourSignatures(
        urlDepot: urlDepot, clePubliqueHex: clePubliqueHex);
  }

  @override
  Future<String> integriteBinaire() async {
    await _ffi();
    return noyau.integriteBinaire();
  }

  @override
  Future<String> calculerSha256(String chemin) async {
    await _ffi();
    return noyau.calculerSha256(chemin: chemin);
  }

  @override
  Future<void> libererRessources() async {
    await _ffi();
    noyau.libererRessources();
  }

  @override
  Future<bool> modeJeu(bool actif) async {
    await _ffi();
    return noyau.modeJeu(actif: actif);
  }

  @override
  Future<bool> modeJeuActif() async {
    await _ffi();
    return noyau.modeJeuActif();
  }

  @override
  Future<ModeDecision> definirMode(ModeDecision mode) async {
    await _ffi();
    return _mode(noyau.definirMode(mode: _modeNatif(mode)));
  }

  @override
  Future<ModeDecision> modeActuel() async {
    await _ffi();
    return _mode(noyau.modeActuel());
  }

  noyau_m.ModeDecision _modeNatif(ModeDecision mode) {
    switch (mode) {
      case ModeDecision.prudent:
        return noyau_m.ModeDecision.prudent;
      case ModeDecision.automatique:
        return noyau_m.ModeDecision.automatique;
      case ModeDecision.agressif:
        return noyau_m.ModeDecision.agressif;
      case ModeDecision.silencieux:
        return noyau_m.ModeDecision.silencieux;
    }
  }

  ModeDecision _mode(noyau_m.ModeDecision mode) {
    switch (mode) {
      case noyau_m.ModeDecision.prudent:
        return ModeDecision.prudent;
      case noyau_m.ModeDecision.automatique:
        return ModeDecision.automatique;
      case noyau_m.ModeDecision.agressif:
        return ModeDecision.agressif;
      case noyau_m.ModeDecision.silencieux:
        return ModeDecision.silencieux;
    }
  }

  @override
  Future<List<ProcessusDto>> listerProcessus() async {
    await _ffi();
    return noyau.analyserProcessusApi().map(_processus).toList();
  }

  ProcessusDto _processus(noyau_r.ProcessusAnalyse p) => ProcessusDto(
        pid: p.pid,
        nom: p.nom,
        exe: p.exe,
        memoire: p.memoire.toInt(),
        signaux: p.signaux,
      );

  @override
  Future<List<String>> limitesRootkit() async {
    await _ffi();
    return noyau.limitesRootkitApi();
  }

  @override
  Future<AnalyseHeuristiqueDto> analyserHeuristique(String chemin) async {
    await _ffi();
    final a = noyau.analyserHeuristiqueApi(chemin: chemin);
    return AnalyseHeuristiqueDto(
      score: a.score,
      signaux: a.signaux,
      verdict: switch (a.verdict) {
        noyau_h.VerdictHeuristique.sain => VerdictHeuristique.sain,
        noyau_h.VerdictHeuristique.suspect => VerdictHeuristique.suspect,
        noyau_h.VerdictHeuristique.menace => VerdictHeuristique.menace,
      },
    );
  }
}
