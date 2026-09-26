/// Contrat du moteur CleanX + DTOs partagés.
///
/// Le contrat est implémenté par :
/// - [MoteurSimule] (démo locale + tests, aucune lib native requise) ;
/// - le pont FRB (`moteur_frb.dart`, après `flutter_rust_bridge_codegen generate`).
/// Les DTOs miment à l'identique les types Rust de `core/src/api.rs`.
library;

/// État global pour le dashboard.
class StatutMoteur {
  final bool protection;
  final int signatures;
  final int menaces;
  final int fichiersAnalyses;
  final bool scanEnCours;
  final List<String> dossiers;

  const StatutMoteur({
    required this.protection,
    required this.signatures,
    required this.menaces,
    required this.fichiersAnalyses,
    required this.scanEnCours,
    required this.dossiers,
  });
}

/// Verdict heuristique.
enum VerdictHeuristique { sain, suspect, menace }

/// Mode de décision face à une menace (P14/ADR-012).
/// **Prudent est le défaut absolu** : détection + notification, jamais d'action auto.
enum ModeDecision { prudent, automatique, agressif, silencieux }

/// Événements poussés du moteur vers l'UI.
sealed class EvenementMoteur {
  const EvenementMoteur();
}

/// Ligne de journal.
class JournalMoteur extends EvenementMoteur {
  final String message;
  const JournalMoteur(this.message);
}

/// Avancement d'un scan.
class ProgressionMoteur extends EvenementMoteur {
  final String scanId;
  final String fichier;
  final int traites;
  final int total;
  const ProgressionMoteur({
    required this.scanId,
    required this.fichier,
    required this.traites,
    required this.total,
  });

  double get ratio => total > 0 ? traites / total : 0;
}

/// Menace détectée (explicable : score + signaux + criticité + confiance,
/// P15/P16/P18/B14). `confiance` 0–100 : 100 = hash confirmé, 70 = motif
/// générique, 30–50 = heuristique seule.
class MenaceMoteur extends EvenementMoteur {
  final String fichier;
  final String menace;
  final String action;
  final String? sha256;
  final int score;
  final List<String> signaux;
  final bool critique;
  final int confiance;
  const MenaceMoteur({
    required this.fichier,
    required this.menace,
    required this.action,
    this.sha256,
    this.score = 0,
    this.signaux = const [],
    this.critique = false,
    this.confiance = 0,
  });
}

/// Fin de scan.
class ScanTermineMoteur extends EvenementMoteur {
  final String scanId;
  final int total;
  final int menaces;
  final bool annule;
  const ScanTermineMoteur({
    required this.scanId,
    required this.total,
    required this.menaces,
    required this.annule,
  });
}

/// Changement d'état de la protection.
class ProtectionMoteur extends EvenementMoteur {
  final bool active;
  const ProtectionMoteur(this.active);
}

/// Fichier isolé.
class FichierQuarantaineDto {
  final int id;
  final String nom;
  final String origine;
  final String date;
  final String raison;
  final int score;
  const FichierQuarantaineDto({
    required this.id,
    required this.nom,
    required this.origine,
    required this.date,
    required this.raison,
    required this.score,
  });
}

/// Entrée de journal persistée.
class EntreeLogDto {
  final int id;
  final String date;
  final String niveau; // info | alerte | quarantaine
  final String message;
  const EntreeLogDto({
    required this.id,
    required this.date,
    required this.niveau,
    required this.message,
  });
}

/// Planification de scan.
class PlanificationDto {
  final int id;
  final String nom;
  final List<String> racines;
  final int intervalleSecs;
  final int prochaineExec;
  const PlanificationDto({
    required this.id,
    required this.nom,
    required this.racines,
    required this.intervalleSecs,
    required this.prochaineExec,
  });
}

/// Résultat d'analyse heuristique.
class AnalyseHeuristiqueDto {
  final int score;
  final List<String> signaux;
  final VerdictHeuristique verdict;
  const AnalyseHeuristiqueDto({
    required this.score,
    required this.signaux,
    required this.verdict,
  });
}

/// Processus analysé (base anti-rootkit userland v1).
class ProcessusDto {
  final int pid;
  final String nom;
  final String? exe;
  final int memoire;
  final List<String> signaux;
  const ProcessusDto({
    required this.pid,
    required this.nom,
    this.exe,
    required this.memoire,
    required this.signaux,
  });

  bool get suspect => signaux.isNotEmpty;
}

/// Contrat moteur. Toute erreur remonte en exception avec message français.
abstract class MoteurCleanX {
  /// Initialise le moteur (`base` = dossier applicatif). Idempotent.
  Future<StatutMoteur> initialiser(String base);
  Future<StatutMoteur> statut();
  Stream<EvenementMoteur> scanRapide();
  Stream<EvenementMoteur> scanComplet();
  Stream<EvenementMoteur> scanPersonnalise(List<String> racines);
  Future<bool> arreterScan();
  Future<void> suspendreScan(bool suspendre);
  /// Active la surveillance (stream infini jusqu'à [desactiverProtection]).
  Stream<EvenementMoteur> activerProtection();
  Future<bool> desactiverProtection();
  Future<List<String>> dossiersSurveilles();
  Future<List<String>> ajouterDossier(String dossier);
  Future<List<String>> retirerDossier(String dossier);
  Future<int> mettreEnQuarantaine(String chemin, String raison);
  Future<List<FichierQuarantaineDto>> listerQuarantaine();
  Future<void> restaurerQuarantaine(int id, String destination);
  Future<void> supprimerQuarantaine(int id);
  Future<List<EntreeLogDto>> listerLogs(int limite);
  Future<String> exporterLogsCsv();
  Future<int> ajouterPlanification(
      String nom, List<String> racines, int intervalleSecs);
  Future<List<PlanificationDto>> listerPlanifications();
  Future<void> supprimerPlanification(int id);
  Future<int> mettreAJourSignatures(String urlDepot, String clePubliqueHex);
  Future<String> integriteBinaire();
  Future<String> calculerSha256(String chemin);
  Future<AnalyseHeuristiqueDto> analyserHeuristique(String chemin);
  /// Libère les ressources natives (pool SQLite). Avant suppression du
  /// dossier de base ou à l'arrêt de l'app.
  Future<void> libererRessources();
  /// Mode jeu : true = scans suspendus + surveillance sans quarantaine auto.
  /// Retourne l'état précédent.
  Future<bool> modeJeu(bool actif);
  Future<bool> modeJeuActif();
  /// Inventaire processus + signaux (base anti-rootkit, voir limites).
  Future<List<ProcessusDto>> listerProcessus();
  Future<List<String>> limitesRootkit();
  /// Définit le mode de décision (opt-in explicites). Retourne le précédent.
  Future<ModeDecision> definirMode(ModeDecision mode);
  /// Mode de décision courant (Prudent si jamais configuré).
  Future<ModeDecision> modeActuel();
}
