// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for French (`fr`).
class AppLocalizationsFr extends AppLocalizations {
  AppLocalizationsFr([String locale = 'fr']) : super(locale);

  @override
  String get appTitle => 'CleanX Antivirus';

  @override
  String get dashboard => 'Tableau de bord';

  @override
  String get scan => 'Analyse';

  @override
  String get protection => 'Protection';

  @override
  String get quarantine => 'Quarantaine';

  @override
  String get logs => 'Journal';

  @override
  String get settings => 'Paramètres';

  @override
  String get protected => 'Votre appareil est protégé';

  @override
  String get unprotected => 'Protection désactivée';

  @override
  String get threatFound => 'Menace détectée';

  @override
  String get securityScore => 'Score de sécurité';

  @override
  String get quickScan => 'Scan Rapide';

  @override
  String get fullScan => 'Scan Complet';

  @override
  String get customScan => 'Scan Personnalisé';

  @override
  String get startScan => 'Lancer l\'analyse';

  @override
  String get stopScan => 'Arrêter le scan';

  @override
  String get pauseScan => 'Pause';

  @override
  String get resumeScan => 'Reprendre';

  @override
  String get waitingScan => 'En attente de scan…';

  @override
  String get fileInProgress => 'Fichier en cours';

  @override
  String get lastScan => 'Dernière analyse';

  @override
  String get signatures => 'Signatures';

  @override
  String get threats => 'Menaces';

  @override
  String get filesAnalyzed => 'Fichiers analysés';

  @override
  String get realtimeProtection => 'Protection en temps réel';

  @override
  String get watchedFolders => 'Dossiers surveillés';

  @override
  String get addFolder => 'Ajouter un dossier';

  @override
  String get folderPath => 'Chemin du dossier';

  @override
  String get add => 'Ajouter';

  @override
  String get remove => 'Retirer';

  @override
  String get restore => 'Restaurer';

  @override
  String get delete => 'Supprimer';

  @override
  String get export => 'Exporter';

  @override
  String get exportCsv => 'Exporter CSV';

  @override
  String get exportPdf => 'Rapport PDF';

  @override
  String get search => 'Rechercher…';

  @override
  String get filterAll => 'Tous';

  @override
  String get noItems => 'Aucun élément pour le moment.';

  @override
  String get theme => 'Thème';

  @override
  String get themeSystem => 'Système';

  @override
  String get themeLight => 'Clair';

  @override
  String get themeDark => 'Sombre';

  @override
  String get language => 'Langue';

  @override
  String get scheduling => 'Planification';

  @override
  String get updates => 'Mises à jour';

  @override
  String get updateSignatures => 'Mettre à jour les signatures';

  @override
  String get modeJeu => 'Mode jeu';

  @override
  String get modeJeuDesc => 'Scans suspendus, détection sans quarantaine auto';

  @override
  String get processusSuspects => 'Processus suspects';

  @override
  String get depotUrl => 'URL du dépôt';

  @override
  String get clePublique => 'Clé publique Ed25519 (hex)';

  @override
  String get about => 'À propos';

  @override
  String get aboutText => 'CleanX 2.0 — moteur Rust natif, interface Flutter.';

  @override
  String get binaryFingerprint => 'Empreinte du binaire';

  @override
  String get confirm => 'Confirmer';

  @override
  String get cancel => 'Annuler';

  @override
  String get ok => 'OK';

  @override
  String get retry => 'Réessayer';

  @override
  String get engineOffline => 'Moteur injoignable — mode démo local';

  @override
  String get scanDone => 'Analyse terminée';

  @override
  String get scanStopped => 'Analyse arrêtée';

  @override
  String get quarantineEmpty => 'Quarantaine vide.';

  @override
  String get restoreConfirm =>
      'Restaurer ce fichier à son emplacement d\'origine ?';

  @override
  String get deleteConfirm => 'Supprimer définitivement ce fichier ?';

  @override
  String get planName => 'Nom de la planification';

  @override
  String get intervalHours => 'Intervalle (heures)';

  @override
  String get rootsLabel => 'Dossiers (séparés par ;)';

  @override
  String get never => 'Jamais';

  @override
  String get scheduledFor => 'Prochain passage';
}
