#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerReadback {
    schema: String,
    status: String,
    route: String,
    authority: String,
    root_manifest_schema: PremiumRootManifestSchema,
    component_manifest_schema: PremiumComponentManifestSchema,
    file_operation_schema: PremiumFileOperationSchema,
    validation_phases: Vec<InstallerPhase>,
    install_phases: Vec<InstallerPhase>,
    rollback_law: RollbackLaw,
    lifecycle_law: Vec<InstallerLifecycleLaw>,
    lane_mapping: Vec<InstallerLaneMapping>,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumRootManifestSchema {
    required_fields: Vec<String>,
    config_fields: Vec<String>,
    file_sections: Vec<String>,
    sample_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumComponentManifestSchema {
    loci: Vec<String>,
    fields: Vec<String>,
    operation_types: Vec<String>,
    blueprint_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumFileOperationSchema {
    source_field: String,
    target_field: String,
    operation_type_field: String,
    identifier_field: String,
    marker_field: String,
    description_field: String,
    supported_operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerPhase {
    id: String,
    sequence: u64,
    source_law: String,
    coronatio_contract: String,
    mutation_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RollbackLaw {
    schema: String,
    order: Vec<String>,
    config_restore: String,
    file_operation_reversal: String,
    service_state_restore: String,
    batch_restore: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerLifecycleLaw {
    action: String,
    sequence: Vec<String>,
    post_build_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerLaneMapping {
    install_mode: InstallMode,
    accepted_package: String,
    post_install_requirement: String,
    rejected_shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RegistryTransactionReadback {
    schema: String,
    status: String,
    route: String,
    source_contract: String,
    transaction_sequence: Vec<RegistryTransactionPhase>,
    deep_merge_law: DeepMergeLaw,
    starred_tab_law: StarredTabLaw,
    validation_law: ConfigValidationLaw,
    persistence_law: ConfigPersistenceLaw,
    rollback_law: ConfigRollbackLaw,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RegistryTransactionPhase {
    sequence: u64,
    id: String,
    source_law: String,
    coronatio_contract: String,
    mutation_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DeepMergeLaw {
    object_merge: String,
    scalar_merge: String,
    array_merge: String,
    tab_merge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StarredTabLaw {
    source_behavior: String,
    preservation_rule: String,
    invalid_starred_resolution: String,
    transaction_requirement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigValidationLaw {
    syntax_gate: String,
    factory_fallback_gate: String,
    temp_validation: String,
    failure_posture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigPersistenceLaw {
    backup_policy: String,
    write_policy: String,
    permission_restore: String,
    missing_config_fallback: String,
    read_only_factory_posture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigRollbackLaw {
    backup_restore: String,
    patch_revert: String,
    complete_tab_removal: String,
    mismatch_policy: String,
}

