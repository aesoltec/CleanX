// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'CleanX Antivirus';

  @override
  String get dashboard => 'Dashboard';

  @override
  String get scan => 'Scan';

  @override
  String get protection => 'Protection';

  @override
  String get quarantine => 'Quarantine';

  @override
  String get logs => 'Logs';

  @override
  String get settings => 'Settings';

  @override
  String get protected => 'Your device is protected';

  @override
  String get unprotected => 'Protection disabled';

  @override
  String get threatFound => 'Threat detected';

  @override
  String get securityScore => 'Security score';

  @override
  String get quickScan => 'Quick Scan';

  @override
  String get fullScan => 'Full Scan';

  @override
  String get customScan => 'Custom Scan';

  @override
  String get startScan => 'Start scan';

  @override
  String get stopScan => 'Stop scan';

  @override
  String get pauseScan => 'Pause';

  @override
  String get resumeScan => 'Resume';

  @override
  String get waitingScan => 'Waiting for scan…';

  @override
  String get fileInProgress => 'Current file';

  @override
  String get lastScan => 'Last scan';

  @override
  String get signatures => 'Signatures';

  @override
  String get threats => 'Threats';

  @override
  String get filesAnalyzed => 'Files analyzed';

  @override
  String get realtimeProtection => 'Real-time protection';

  @override
  String get watchedFolders => 'Watched folders';

  @override
  String get addFolder => 'Add a folder';

  @override
  String get folderPath => 'Folder path';

  @override
  String get add => 'Add';

  @override
  String get remove => 'Remove';

  @override
  String get restore => 'Restore';

  @override
  String get delete => 'Delete';

  @override
  String get export => 'Export';

  @override
  String get exportCsv => 'Export CSV';

  @override
  String get exportPdf => 'PDF report';

  @override
  String get search => 'Search…';

  @override
  String get filterAll => 'All';

  @override
  String get noItems => 'No items yet.';

  @override
  String get theme => 'Theme';

  @override
  String get themeSystem => 'System';

  @override
  String get themeLight => 'Light';

  @override
  String get themeDark => 'Dark';

  @override
  String get language => 'Language';

  @override
  String get scheduling => 'Scheduling';

  @override
  String get updates => 'Updates';

  @override
  String get updateSignatures => 'Update signatures';

  @override
  String get modeJeu => 'Gaming mode';

  @override
  String get modeJeuDesc => 'Scans paused, detection without auto-quarantine';

  @override
  String get modeDecision => 'Decision mode';

  @override
  String get modePrudent => 'Cautious (recommended)';

  @override
  String get modePrudentDesc =>
      'Asks before any action. No automatic quarantine.';

  @override
  String get modeAuto => 'Automatic';

  @override
  String get modeAutoDesc =>
      'Auto-quarantine on confirmed threat. Enable explicitly.';

  @override
  String get modeAgressif => 'Aggressive';

  @override
  String get modeAgressifDesc =>
      'Auto-quarantine from score ≥ 50. Never deletes.';

  @override
  String get modeSilencieux => 'Silent';

  @override
  String get modeSilencieuxDesc => 'Logs only, takes no action.';

  @override
  String get processusSuspects => 'Suspicious processes';

  @override
  String get depotUrl => 'Repository URL';

  @override
  String get clePublique => 'Ed25519 public key (hex)';

  @override
  String get about => 'About';

  @override
  String get aboutText => 'CleanX 2.0 — native Rust engine, Flutter UI.';

  @override
  String get binaryFingerprint => 'Binary fingerprint';

  @override
  String get confirm => 'Confirm';

  @override
  String get cancel => 'Cancel';

  @override
  String get ok => 'OK';

  @override
  String get retry => 'Retry';

  @override
  String get engineOffline => 'Engine unreachable — local demo mode';

  @override
  String get scanDone => 'Scan finished';

  @override
  String get scanStopped => 'Scan stopped';

  @override
  String get quarantineEmpty => 'Quarantine is empty.';

  @override
  String get restoreConfirm => 'Restore this file to its original location?';

  @override
  String get deleteConfirm => 'Permanently delete this file?';

  @override
  String get planName => 'Schedule name';

  @override
  String get intervalHours => 'Interval (hours)';

  @override
  String get rootsLabel => 'Folders (separated by ;)';

  @override
  String get never => 'Never';

  @override
  String get scheduledFor => 'Next run';
}
