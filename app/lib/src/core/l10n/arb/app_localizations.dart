import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_fr.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'arb/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
      : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
    delegate,
    GlobalMaterialLocalizations.delegate,
    GlobalCupertinoLocalizations.delegate,
    GlobalWidgetsLocalizations.delegate,
  ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('fr')
  ];

  /// No description provided for @appTitle.
  ///
  /// In fr, this message translates to:
  /// **'CleanX Antivirus'**
  String get appTitle;

  /// No description provided for @dashboard.
  ///
  /// In fr, this message translates to:
  /// **'Tableau de bord'**
  String get dashboard;

  /// No description provided for @scan.
  ///
  /// In fr, this message translates to:
  /// **'Analyse'**
  String get scan;

  /// No description provided for @protection.
  ///
  /// In fr, this message translates to:
  /// **'Protection'**
  String get protection;

  /// No description provided for @quarantine.
  ///
  /// In fr, this message translates to:
  /// **'Quarantaine'**
  String get quarantine;

  /// No description provided for @logs.
  ///
  /// In fr, this message translates to:
  /// **'Journal'**
  String get logs;

  /// No description provided for @settings.
  ///
  /// In fr, this message translates to:
  /// **'Paramètres'**
  String get settings;

  /// No description provided for @protected.
  ///
  /// In fr, this message translates to:
  /// **'Votre appareil est protégé'**
  String get protected;

  /// No description provided for @unprotected.
  ///
  /// In fr, this message translates to:
  /// **'Protection désactivée'**
  String get unprotected;

  /// No description provided for @threatFound.
  ///
  /// In fr, this message translates to:
  /// **'Menace détectée'**
  String get threatFound;

  /// No description provided for @securityScore.
  ///
  /// In fr, this message translates to:
  /// **'Score de sécurité'**
  String get securityScore;

  /// No description provided for @quickScan.
  ///
  /// In fr, this message translates to:
  /// **'Scan Rapide'**
  String get quickScan;

  /// No description provided for @fullScan.
  ///
  /// In fr, this message translates to:
  /// **'Scan Complet'**
  String get fullScan;

  /// No description provided for @customScan.
  ///
  /// In fr, this message translates to:
  /// **'Scan Personnalisé'**
  String get customScan;

  /// No description provided for @startScan.
  ///
  /// In fr, this message translates to:
  /// **'Lancer l\'analyse'**
  String get startScan;

  /// No description provided for @stopScan.
  ///
  /// In fr, this message translates to:
  /// **'Arrêter le scan'**
  String get stopScan;

  /// No description provided for @pauseScan.
  ///
  /// In fr, this message translates to:
  /// **'Pause'**
  String get pauseScan;

  /// No description provided for @resumeScan.
  ///
  /// In fr, this message translates to:
  /// **'Reprendre'**
  String get resumeScan;

  /// No description provided for @waitingScan.
  ///
  /// In fr, this message translates to:
  /// **'En attente de scan…'**
  String get waitingScan;

  /// No description provided for @fileInProgress.
  ///
  /// In fr, this message translates to:
  /// **'Fichier en cours'**
  String get fileInProgress;

  /// No description provided for @lastScan.
  ///
  /// In fr, this message translates to:
  /// **'Dernière analyse'**
  String get lastScan;

  /// No description provided for @signatures.
  ///
  /// In fr, this message translates to:
  /// **'Signatures'**
  String get signatures;

  /// No description provided for @threats.
  ///
  /// In fr, this message translates to:
  /// **'Menaces'**
  String get threats;

  /// No description provided for @filesAnalyzed.
  ///
  /// In fr, this message translates to:
  /// **'Fichiers analysés'**
  String get filesAnalyzed;

  /// No description provided for @realtimeProtection.
  ///
  /// In fr, this message translates to:
  /// **'Protection en temps réel'**
  String get realtimeProtection;

  /// No description provided for @watchedFolders.
  ///
  /// In fr, this message translates to:
  /// **'Dossiers surveillés'**
  String get watchedFolders;

  /// No description provided for @addFolder.
  ///
  /// In fr, this message translates to:
  /// **'Ajouter un dossier'**
  String get addFolder;

  /// No description provided for @folderPath.
  ///
  /// In fr, this message translates to:
  /// **'Chemin du dossier'**
  String get folderPath;

  /// No description provided for @add.
  ///
  /// In fr, this message translates to:
  /// **'Ajouter'**
  String get add;

  /// No description provided for @remove.
  ///
  /// In fr, this message translates to:
  /// **'Retirer'**
  String get remove;

  /// No description provided for @restore.
  ///
  /// In fr, this message translates to:
  /// **'Restaurer'**
  String get restore;

  /// No description provided for @delete.
  ///
  /// In fr, this message translates to:
  /// **'Supprimer'**
  String get delete;

  /// No description provided for @export.
  ///
  /// In fr, this message translates to:
  /// **'Exporter'**
  String get export;

  /// No description provided for @exportCsv.
  ///
  /// In fr, this message translates to:
  /// **'Exporter CSV'**
  String get exportCsv;

  /// No description provided for @exportPdf.
  ///
  /// In fr, this message translates to:
  /// **'Rapport PDF'**
  String get exportPdf;

  /// No description provided for @search.
  ///
  /// In fr, this message translates to:
  /// **'Rechercher…'**
  String get search;

  /// No description provided for @filterAll.
  ///
  /// In fr, this message translates to:
  /// **'Tous'**
  String get filterAll;

  /// No description provided for @noItems.
  ///
  /// In fr, this message translates to:
  /// **'Aucun élément pour le moment.'**
  String get noItems;

  /// No description provided for @theme.
  ///
  /// In fr, this message translates to:
  /// **'Thème'**
  String get theme;

  /// No description provided for @themeSystem.
  ///
  /// In fr, this message translates to:
  /// **'Système'**
  String get themeSystem;

  /// No description provided for @themeLight.
  ///
  /// In fr, this message translates to:
  /// **'Clair'**
  String get themeLight;

  /// No description provided for @themeDark.
  ///
  /// In fr, this message translates to:
  /// **'Sombre'**
  String get themeDark;

  /// No description provided for @language.
  ///
  /// In fr, this message translates to:
  /// **'Langue'**
  String get language;

  /// No description provided for @scheduling.
  ///
  /// In fr, this message translates to:
  /// **'Planification'**
  String get scheduling;

  /// No description provided for @updates.
  ///
  /// In fr, this message translates to:
  /// **'Mises à jour'**
  String get updates;

  /// No description provided for @updateSignatures.
  ///
  /// In fr, this message translates to:
  /// **'Mettre à jour les signatures'**
  String get updateSignatures;

  /// No description provided for @modeJeu.
  ///
  /// In fr, this message translates to:
  /// **'Mode jeu'**
  String get modeJeu;

  /// No description provided for @modeJeuDesc.
  ///
  /// In fr, this message translates to:
  /// **'Scans suspendus, détection sans quarantaine auto'**
  String get modeJeuDesc;

  /// No description provided for @processusSuspects.
  ///
  /// In fr, this message translates to:
  /// **'Processus suspects'**
  String get processusSuspects;

  /// No description provided for @depotUrl.
  ///
  /// In fr, this message translates to:
  /// **'URL du dépôt'**
  String get depotUrl;

  /// No description provided for @clePublique.
  ///
  /// In fr, this message translates to:
  /// **'Clé publique Ed25519 (hex)'**
  String get clePublique;

  /// No description provided for @about.
  ///
  /// In fr, this message translates to:
  /// **'À propos'**
  String get about;

  /// No description provided for @aboutText.
  ///
  /// In fr, this message translates to:
  /// **'CleanX 2.0 — moteur Rust natif, interface Flutter.'**
  String get aboutText;

  /// No description provided for @binaryFingerprint.
  ///
  /// In fr, this message translates to:
  /// **'Empreinte du binaire'**
  String get binaryFingerprint;

  /// No description provided for @confirm.
  ///
  /// In fr, this message translates to:
  /// **'Confirmer'**
  String get confirm;

  /// No description provided for @cancel.
  ///
  /// In fr, this message translates to:
  /// **'Annuler'**
  String get cancel;

  /// No description provided for @ok.
  ///
  /// In fr, this message translates to:
  /// **'OK'**
  String get ok;

  /// No description provided for @retry.
  ///
  /// In fr, this message translates to:
  /// **'Réessayer'**
  String get retry;

  /// No description provided for @engineOffline.
  ///
  /// In fr, this message translates to:
  /// **'Moteur injoignable — mode démo local'**
  String get engineOffline;

  /// No description provided for @scanDone.
  ///
  /// In fr, this message translates to:
  /// **'Analyse terminée'**
  String get scanDone;

  /// No description provided for @scanStopped.
  ///
  /// In fr, this message translates to:
  /// **'Analyse arrêtée'**
  String get scanStopped;

  /// No description provided for @quarantineEmpty.
  ///
  /// In fr, this message translates to:
  /// **'Quarantaine vide.'**
  String get quarantineEmpty;

  /// No description provided for @restoreConfirm.
  ///
  /// In fr, this message translates to:
  /// **'Restaurer ce fichier à son emplacement d\'origine ?'**
  String get restoreConfirm;

  /// No description provided for @deleteConfirm.
  ///
  /// In fr, this message translates to:
  /// **'Supprimer définitivement ce fichier ?'**
  String get deleteConfirm;

  /// No description provided for @planName.
  ///
  /// In fr, this message translates to:
  /// **'Nom de la planification'**
  String get planName;

  /// No description provided for @intervalHours.
  ///
  /// In fr, this message translates to:
  /// **'Intervalle (heures)'**
  String get intervalHours;

  /// No description provided for @rootsLabel.
  ///
  /// In fr, this message translates to:
  /// **'Dossiers (séparés par ;)'**
  String get rootsLabel;

  /// No description provided for @never.
  ///
  /// In fr, this message translates to:
  /// **'Jamais'**
  String get never;

  /// No description provided for @scheduledFor.
  ///
  /// In fr, this message translates to:
  /// **'Prochain passage'**
  String get scheduledFor;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'fr'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'fr':
      return AppLocalizationsFr();
  }

  throw FlutterError(
      'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
      'an issue with the localizations generation tool. Please file an issue '
      'on GitHub with a reproducible sample app and the gen-l10n configuration '
      'that was used.');
}
