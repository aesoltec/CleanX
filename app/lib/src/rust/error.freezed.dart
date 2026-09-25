// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'error.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CleanXError {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is CleanXError);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'CleanXError()';
  }
}

/// @nodoc
class $CleanXErrorCopyWith<$Res> {
  $CleanXErrorCopyWith(CleanXError _, $Res Function(CleanXError) __);
}

/// Adds pattern-matching-related methods to [CleanXError].
extension CleanXErrorPatterns on CleanXError {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CleanXError_MoteurNonInitialise value)?
        moteurNonInitialise,
    TResult Function(CleanXError_FichierIntrouvable value)? fichierIntrouvable,
    TResult Function(CleanXError_PermissionRefusee value)? permissionRefusee,
    TResult Function(CleanXError_FichierVerrouille value)? fichierVerrouille,
    TResult Function(CleanXError_CheminInvalide value)? cheminInvalide,
    TResult Function(CleanXError_BaseCorrompue value)? baseCorrompue,
    TResult Function(CleanXError_Quarantaine value)? quarantaine,
    TResult Function(CleanXError_Surveillance value)? surveillance,
    TResult Function(CleanXError_ScanAnnule value)? scanAnnule,
    TResult Function(CleanXError_ScanDejaEnCours value)? scanDejaEnCours,
    TResult Function(CleanXError_NonSupporte value)? nonSupporte,
    TResult Function(CleanXError_Interne value)? interne,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise() when moteurNonInitialise != null:
        return moteurNonInitialise(_that);
      case CleanXError_FichierIntrouvable() when fichierIntrouvable != null:
        return fichierIntrouvable(_that);
      case CleanXError_PermissionRefusee() when permissionRefusee != null:
        return permissionRefusee(_that);
      case CleanXError_FichierVerrouille() when fichierVerrouille != null:
        return fichierVerrouille(_that);
      case CleanXError_CheminInvalide() when cheminInvalide != null:
        return cheminInvalide(_that);
      case CleanXError_BaseCorrompue() when baseCorrompue != null:
        return baseCorrompue(_that);
      case CleanXError_Quarantaine() when quarantaine != null:
        return quarantaine(_that);
      case CleanXError_Surveillance() when surveillance != null:
        return surveillance(_that);
      case CleanXError_ScanAnnule() when scanAnnule != null:
        return scanAnnule(_that);
      case CleanXError_ScanDejaEnCours() when scanDejaEnCours != null:
        return scanDejaEnCours(_that);
      case CleanXError_NonSupporte() when nonSupporte != null:
        return nonSupporte(_that);
      case CleanXError_Interne() when interne != null:
        return interne(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CleanXError_MoteurNonInitialise value)
        moteurNonInitialise,
    required TResult Function(CleanXError_FichierIntrouvable value)
        fichierIntrouvable,
    required TResult Function(CleanXError_PermissionRefusee value)
        permissionRefusee,
    required TResult Function(CleanXError_FichierVerrouille value)
        fichierVerrouille,
    required TResult Function(CleanXError_CheminInvalide value) cheminInvalide,
    required TResult Function(CleanXError_BaseCorrompue value) baseCorrompue,
    required TResult Function(CleanXError_Quarantaine value) quarantaine,
    required TResult Function(CleanXError_Surveillance value) surveillance,
    required TResult Function(CleanXError_ScanAnnule value) scanAnnule,
    required TResult Function(CleanXError_ScanDejaEnCours value)
        scanDejaEnCours,
    required TResult Function(CleanXError_NonSupporte value) nonSupporte,
    required TResult Function(CleanXError_Interne value) interne,
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise():
        return moteurNonInitialise(_that);
      case CleanXError_FichierIntrouvable():
        return fichierIntrouvable(_that);
      case CleanXError_PermissionRefusee():
        return permissionRefusee(_that);
      case CleanXError_FichierVerrouille():
        return fichierVerrouille(_that);
      case CleanXError_CheminInvalide():
        return cheminInvalide(_that);
      case CleanXError_BaseCorrompue():
        return baseCorrompue(_that);
      case CleanXError_Quarantaine():
        return quarantaine(_that);
      case CleanXError_Surveillance():
        return surveillance(_that);
      case CleanXError_ScanAnnule():
        return scanAnnule(_that);
      case CleanXError_ScanDejaEnCours():
        return scanDejaEnCours(_that);
      case CleanXError_NonSupporte():
        return nonSupporte(_that);
      case CleanXError_Interne():
        return interne(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CleanXError_MoteurNonInitialise value)?
        moteurNonInitialise,
    TResult? Function(CleanXError_FichierIntrouvable value)? fichierIntrouvable,
    TResult? Function(CleanXError_PermissionRefusee value)? permissionRefusee,
    TResult? Function(CleanXError_FichierVerrouille value)? fichierVerrouille,
    TResult? Function(CleanXError_CheminInvalide value)? cheminInvalide,
    TResult? Function(CleanXError_BaseCorrompue value)? baseCorrompue,
    TResult? Function(CleanXError_Quarantaine value)? quarantaine,
    TResult? Function(CleanXError_Surveillance value)? surveillance,
    TResult? Function(CleanXError_ScanAnnule value)? scanAnnule,
    TResult? Function(CleanXError_ScanDejaEnCours value)? scanDejaEnCours,
    TResult? Function(CleanXError_NonSupporte value)? nonSupporte,
    TResult? Function(CleanXError_Interne value)? interne,
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise() when moteurNonInitialise != null:
        return moteurNonInitialise(_that);
      case CleanXError_FichierIntrouvable() when fichierIntrouvable != null:
        return fichierIntrouvable(_that);
      case CleanXError_PermissionRefusee() when permissionRefusee != null:
        return permissionRefusee(_that);
      case CleanXError_FichierVerrouille() when fichierVerrouille != null:
        return fichierVerrouille(_that);
      case CleanXError_CheminInvalide() when cheminInvalide != null:
        return cheminInvalide(_that);
      case CleanXError_BaseCorrompue() when baseCorrompue != null:
        return baseCorrompue(_that);
      case CleanXError_Quarantaine() when quarantaine != null:
        return quarantaine(_that);
      case CleanXError_Surveillance() when surveillance != null:
        return surveillance(_that);
      case CleanXError_ScanAnnule() when scanAnnule != null:
        return scanAnnule(_that);
      case CleanXError_ScanDejaEnCours() when scanDejaEnCours != null:
        return scanDejaEnCours(_that);
      case CleanXError_NonSupporte() when nonSupporte != null:
        return nonSupporte(_that);
      case CleanXError_Interne() when interne != null:
        return interne(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? moteurNonInitialise,
    TResult Function(String chemin)? fichierIntrouvable,
    TResult Function(String chemin, String detail)? permissionRefusee,
    TResult Function(String chemin)? fichierVerrouille,
    TResult Function(String chemin)? cheminInvalide,
    TResult Function(String detail)? baseCorrompue,
    TResult Function(String detail)? quarantaine,
    TResult Function(String detail)? surveillance,
    TResult Function()? scanAnnule,
    TResult Function()? scanDejaEnCours,
    TResult Function(String detail)? nonSupporte,
    TResult Function(String detail)? interne,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise() when moteurNonInitialise != null:
        return moteurNonInitialise();
      case CleanXError_FichierIntrouvable() when fichierIntrouvable != null:
        return fichierIntrouvable(_that.chemin);
      case CleanXError_PermissionRefusee() when permissionRefusee != null:
        return permissionRefusee(_that.chemin, _that.detail);
      case CleanXError_FichierVerrouille() when fichierVerrouille != null:
        return fichierVerrouille(_that.chemin);
      case CleanXError_CheminInvalide() when cheminInvalide != null:
        return cheminInvalide(_that.chemin);
      case CleanXError_BaseCorrompue() when baseCorrompue != null:
        return baseCorrompue(_that.detail);
      case CleanXError_Quarantaine() when quarantaine != null:
        return quarantaine(_that.detail);
      case CleanXError_Surveillance() when surveillance != null:
        return surveillance(_that.detail);
      case CleanXError_ScanAnnule() when scanAnnule != null:
        return scanAnnule();
      case CleanXError_ScanDejaEnCours() when scanDejaEnCours != null:
        return scanDejaEnCours();
      case CleanXError_NonSupporte() when nonSupporte != null:
        return nonSupporte(_that.detail);
      case CleanXError_Interne() when interne != null:
        return interne(_that.detail);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() moteurNonInitialise,
    required TResult Function(String chemin) fichierIntrouvable,
    required TResult Function(String chemin, String detail) permissionRefusee,
    required TResult Function(String chemin) fichierVerrouille,
    required TResult Function(String chemin) cheminInvalide,
    required TResult Function(String detail) baseCorrompue,
    required TResult Function(String detail) quarantaine,
    required TResult Function(String detail) surveillance,
    required TResult Function() scanAnnule,
    required TResult Function() scanDejaEnCours,
    required TResult Function(String detail) nonSupporte,
    required TResult Function(String detail) interne,
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise():
        return moteurNonInitialise();
      case CleanXError_FichierIntrouvable():
        return fichierIntrouvable(_that.chemin);
      case CleanXError_PermissionRefusee():
        return permissionRefusee(_that.chemin, _that.detail);
      case CleanXError_FichierVerrouille():
        return fichierVerrouille(_that.chemin);
      case CleanXError_CheminInvalide():
        return cheminInvalide(_that.chemin);
      case CleanXError_BaseCorrompue():
        return baseCorrompue(_that.detail);
      case CleanXError_Quarantaine():
        return quarantaine(_that.detail);
      case CleanXError_Surveillance():
        return surveillance(_that.detail);
      case CleanXError_ScanAnnule():
        return scanAnnule();
      case CleanXError_ScanDejaEnCours():
        return scanDejaEnCours();
      case CleanXError_NonSupporte():
        return nonSupporte(_that.detail);
      case CleanXError_Interne():
        return interne(_that.detail);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? moteurNonInitialise,
    TResult? Function(String chemin)? fichierIntrouvable,
    TResult? Function(String chemin, String detail)? permissionRefusee,
    TResult? Function(String chemin)? fichierVerrouille,
    TResult? Function(String chemin)? cheminInvalide,
    TResult? Function(String detail)? baseCorrompue,
    TResult? Function(String detail)? quarantaine,
    TResult? Function(String detail)? surveillance,
    TResult? Function()? scanAnnule,
    TResult? Function()? scanDejaEnCours,
    TResult? Function(String detail)? nonSupporte,
    TResult? Function(String detail)? interne,
  }) {
    final _that = this;
    switch (_that) {
      case CleanXError_MoteurNonInitialise() when moteurNonInitialise != null:
        return moteurNonInitialise();
      case CleanXError_FichierIntrouvable() when fichierIntrouvable != null:
        return fichierIntrouvable(_that.chemin);
      case CleanXError_PermissionRefusee() when permissionRefusee != null:
        return permissionRefusee(_that.chemin, _that.detail);
      case CleanXError_FichierVerrouille() when fichierVerrouille != null:
        return fichierVerrouille(_that.chemin);
      case CleanXError_CheminInvalide() when cheminInvalide != null:
        return cheminInvalide(_that.chemin);
      case CleanXError_BaseCorrompue() when baseCorrompue != null:
        return baseCorrompue(_that.detail);
      case CleanXError_Quarantaine() when quarantaine != null:
        return quarantaine(_that.detail);
      case CleanXError_Surveillance() when surveillance != null:
        return surveillance(_that.detail);
      case CleanXError_ScanAnnule() when scanAnnule != null:
        return scanAnnule();
      case CleanXError_ScanDejaEnCours() when scanDejaEnCours != null:
        return scanDejaEnCours();
      case CleanXError_NonSupporte() when nonSupporte != null:
        return nonSupporte(_that.detail);
      case CleanXError_Interne() when interne != null:
        return interne(_that.detail);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CleanXError_MoteurNonInitialise extends CleanXError {
  const CleanXError_MoteurNonInitialise() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_MoteurNonInitialise);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'CleanXError.moteurNonInitialise()';
  }
}

/// @nodoc

class CleanXError_FichierIntrouvable extends CleanXError {
  const CleanXError_FichierIntrouvable({required this.chemin}) : super._();

  final String chemin;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_FichierIntrouvableCopyWith<CleanXError_FichierIntrouvable>
      get copyWith => _$CleanXError_FichierIntrouvableCopyWithImpl<
          CleanXError_FichierIntrouvable>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_FichierIntrouvable &&
            (identical(other.chemin, chemin) || other.chemin == chemin));
  }

  @override
  int get hashCode => Object.hash(runtimeType, chemin);

  @override
  String toString() {
    return 'CleanXError.fichierIntrouvable(chemin: $chemin)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_FichierIntrouvableCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_FichierIntrouvableCopyWith(
          CleanXError_FichierIntrouvable value,
          $Res Function(CleanXError_FichierIntrouvable) _then) =
      _$CleanXError_FichierIntrouvableCopyWithImpl;
  @useResult
  $Res call({String chemin});
}

/// @nodoc
class _$CleanXError_FichierIntrouvableCopyWithImpl<$Res>
    implements $CleanXError_FichierIntrouvableCopyWith<$Res> {
  _$CleanXError_FichierIntrouvableCopyWithImpl(this._self, this._then);

  final CleanXError_FichierIntrouvable _self;
  final $Res Function(CleanXError_FichierIntrouvable) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? chemin = null,
  }) {
    return _then(CleanXError_FichierIntrouvable(
      chemin: null == chemin
          ? _self.chemin
          : chemin // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_PermissionRefusee extends CleanXError {
  const CleanXError_PermissionRefusee(
      {required this.chemin, required this.detail})
      : super._();

  final String chemin;
  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_PermissionRefuseeCopyWith<CleanXError_PermissionRefusee>
      get copyWith => _$CleanXError_PermissionRefuseeCopyWithImpl<
          CleanXError_PermissionRefusee>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_PermissionRefusee &&
            (identical(other.chemin, chemin) || other.chemin == chemin) &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, chemin, detail);

  @override
  String toString() {
    return 'CleanXError.permissionRefusee(chemin: $chemin, detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_PermissionRefuseeCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_PermissionRefuseeCopyWith(
          CleanXError_PermissionRefusee value,
          $Res Function(CleanXError_PermissionRefusee) _then) =
      _$CleanXError_PermissionRefuseeCopyWithImpl;
  @useResult
  $Res call({String chemin, String detail});
}

/// @nodoc
class _$CleanXError_PermissionRefuseeCopyWithImpl<$Res>
    implements $CleanXError_PermissionRefuseeCopyWith<$Res> {
  _$CleanXError_PermissionRefuseeCopyWithImpl(this._self, this._then);

  final CleanXError_PermissionRefusee _self;
  final $Res Function(CleanXError_PermissionRefusee) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? chemin = null,
    Object? detail = null,
  }) {
    return _then(CleanXError_PermissionRefusee(
      chemin: null == chemin
          ? _self.chemin
          : chemin // ignore: cast_nullable_to_non_nullable
              as String,
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_FichierVerrouille extends CleanXError {
  const CleanXError_FichierVerrouille({required this.chemin}) : super._();

  final String chemin;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_FichierVerrouilleCopyWith<CleanXError_FichierVerrouille>
      get copyWith => _$CleanXError_FichierVerrouilleCopyWithImpl<
          CleanXError_FichierVerrouille>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_FichierVerrouille &&
            (identical(other.chemin, chemin) || other.chemin == chemin));
  }

  @override
  int get hashCode => Object.hash(runtimeType, chemin);

  @override
  String toString() {
    return 'CleanXError.fichierVerrouille(chemin: $chemin)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_FichierVerrouilleCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_FichierVerrouilleCopyWith(
          CleanXError_FichierVerrouille value,
          $Res Function(CleanXError_FichierVerrouille) _then) =
      _$CleanXError_FichierVerrouilleCopyWithImpl;
  @useResult
  $Res call({String chemin});
}

/// @nodoc
class _$CleanXError_FichierVerrouilleCopyWithImpl<$Res>
    implements $CleanXError_FichierVerrouilleCopyWith<$Res> {
  _$CleanXError_FichierVerrouilleCopyWithImpl(this._self, this._then);

  final CleanXError_FichierVerrouille _self;
  final $Res Function(CleanXError_FichierVerrouille) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? chemin = null,
  }) {
    return _then(CleanXError_FichierVerrouille(
      chemin: null == chemin
          ? _self.chemin
          : chemin // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_CheminInvalide extends CleanXError {
  const CleanXError_CheminInvalide({required this.chemin}) : super._();

  final String chemin;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_CheminInvalideCopyWith<CleanXError_CheminInvalide>
      get copyWith =>
          _$CleanXError_CheminInvalideCopyWithImpl<CleanXError_CheminInvalide>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_CheminInvalide &&
            (identical(other.chemin, chemin) || other.chemin == chemin));
  }

  @override
  int get hashCode => Object.hash(runtimeType, chemin);

  @override
  String toString() {
    return 'CleanXError.cheminInvalide(chemin: $chemin)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_CheminInvalideCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_CheminInvalideCopyWith(CleanXError_CheminInvalide value,
          $Res Function(CleanXError_CheminInvalide) _then) =
      _$CleanXError_CheminInvalideCopyWithImpl;
  @useResult
  $Res call({String chemin});
}

/// @nodoc
class _$CleanXError_CheminInvalideCopyWithImpl<$Res>
    implements $CleanXError_CheminInvalideCopyWith<$Res> {
  _$CleanXError_CheminInvalideCopyWithImpl(this._self, this._then);

  final CleanXError_CheminInvalide _self;
  final $Res Function(CleanXError_CheminInvalide) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? chemin = null,
  }) {
    return _then(CleanXError_CheminInvalide(
      chemin: null == chemin
          ? _self.chemin
          : chemin // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_BaseCorrompue extends CleanXError {
  const CleanXError_BaseCorrompue({required this.detail}) : super._();

  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_BaseCorrompueCopyWith<CleanXError_BaseCorrompue> get copyWith =>
      _$CleanXError_BaseCorrompueCopyWithImpl<CleanXError_BaseCorrompue>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_BaseCorrompue &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, detail);

  @override
  String toString() {
    return 'CleanXError.baseCorrompue(detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_BaseCorrompueCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_BaseCorrompueCopyWith(CleanXError_BaseCorrompue value,
          $Res Function(CleanXError_BaseCorrompue) _then) =
      _$CleanXError_BaseCorrompueCopyWithImpl;
  @useResult
  $Res call({String detail});
}

/// @nodoc
class _$CleanXError_BaseCorrompueCopyWithImpl<$Res>
    implements $CleanXError_BaseCorrompueCopyWith<$Res> {
  _$CleanXError_BaseCorrompueCopyWithImpl(this._self, this._then);

  final CleanXError_BaseCorrompue _self;
  final $Res Function(CleanXError_BaseCorrompue) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? detail = null,
  }) {
    return _then(CleanXError_BaseCorrompue(
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_Quarantaine extends CleanXError {
  const CleanXError_Quarantaine({required this.detail}) : super._();

  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_QuarantaineCopyWith<CleanXError_Quarantaine> get copyWith =>
      _$CleanXError_QuarantaineCopyWithImpl<CleanXError_Quarantaine>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_Quarantaine &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, detail);

  @override
  String toString() {
    return 'CleanXError.quarantaine(detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_QuarantaineCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_QuarantaineCopyWith(CleanXError_Quarantaine value,
          $Res Function(CleanXError_Quarantaine) _then) =
      _$CleanXError_QuarantaineCopyWithImpl;
  @useResult
  $Res call({String detail});
}

/// @nodoc
class _$CleanXError_QuarantaineCopyWithImpl<$Res>
    implements $CleanXError_QuarantaineCopyWith<$Res> {
  _$CleanXError_QuarantaineCopyWithImpl(this._self, this._then);

  final CleanXError_Quarantaine _self;
  final $Res Function(CleanXError_Quarantaine) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? detail = null,
  }) {
    return _then(CleanXError_Quarantaine(
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_Surveillance extends CleanXError {
  const CleanXError_Surveillance({required this.detail}) : super._();

  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_SurveillanceCopyWith<CleanXError_Surveillance> get copyWith =>
      _$CleanXError_SurveillanceCopyWithImpl<CleanXError_Surveillance>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_Surveillance &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, detail);

  @override
  String toString() {
    return 'CleanXError.surveillance(detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_SurveillanceCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_SurveillanceCopyWith(CleanXError_Surveillance value,
          $Res Function(CleanXError_Surveillance) _then) =
      _$CleanXError_SurveillanceCopyWithImpl;
  @useResult
  $Res call({String detail});
}

/// @nodoc
class _$CleanXError_SurveillanceCopyWithImpl<$Res>
    implements $CleanXError_SurveillanceCopyWith<$Res> {
  _$CleanXError_SurveillanceCopyWithImpl(this._self, this._then);

  final CleanXError_Surveillance _self;
  final $Res Function(CleanXError_Surveillance) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? detail = null,
  }) {
    return _then(CleanXError_Surveillance(
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_ScanAnnule extends CleanXError {
  const CleanXError_ScanAnnule() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is CleanXError_ScanAnnule);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'CleanXError.scanAnnule()';
  }
}

/// @nodoc

class CleanXError_ScanDejaEnCours extends CleanXError {
  const CleanXError_ScanDejaEnCours() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_ScanDejaEnCours);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'CleanXError.scanDejaEnCours()';
  }
}

/// @nodoc

class CleanXError_NonSupporte extends CleanXError {
  const CleanXError_NonSupporte({required this.detail}) : super._();

  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_NonSupporteCopyWith<CleanXError_NonSupporte> get copyWith =>
      _$CleanXError_NonSupporteCopyWithImpl<CleanXError_NonSupporte>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_NonSupporte &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, detail);

  @override
  String toString() {
    return 'CleanXError.nonSupporte(detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_NonSupporteCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_NonSupporteCopyWith(CleanXError_NonSupporte value,
          $Res Function(CleanXError_NonSupporte) _then) =
      _$CleanXError_NonSupporteCopyWithImpl;
  @useResult
  $Res call({String detail});
}

/// @nodoc
class _$CleanXError_NonSupporteCopyWithImpl<$Res>
    implements $CleanXError_NonSupporteCopyWith<$Res> {
  _$CleanXError_NonSupporteCopyWithImpl(this._self, this._then);

  final CleanXError_NonSupporte _self;
  final $Res Function(CleanXError_NonSupporte) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? detail = null,
  }) {
    return _then(CleanXError_NonSupporte(
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CleanXError_Interne extends CleanXError {
  const CleanXError_Interne({required this.detail}) : super._();

  final String detail;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CleanXError_InterneCopyWith<CleanXError_Interne> get copyWith =>
      _$CleanXError_InterneCopyWithImpl<CleanXError_Interne>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CleanXError_Interne &&
            (identical(other.detail, detail) || other.detail == detail));
  }

  @override
  int get hashCode => Object.hash(runtimeType, detail);

  @override
  String toString() {
    return 'CleanXError.interne(detail: $detail)';
  }
}

/// @nodoc
abstract mixin class $CleanXError_InterneCopyWith<$Res>
    implements $CleanXErrorCopyWith<$Res> {
  factory $CleanXError_InterneCopyWith(
          CleanXError_Interne value, $Res Function(CleanXError_Interne) _then) =
      _$CleanXError_InterneCopyWithImpl;
  @useResult
  $Res call({String detail});
}

/// @nodoc
class _$CleanXError_InterneCopyWithImpl<$Res>
    implements $CleanXError_InterneCopyWith<$Res> {
  _$CleanXError_InterneCopyWithImpl(this._self, this._then);

  final CleanXError_Interne _self;
  final $Res Function(CleanXError_Interne) _then;

  /// Create a copy of CleanXError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? detail = null,
  }) {
    return _then(CleanXError_Interne(
      detail: null == detail
          ? _self.detail
          : detail // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
