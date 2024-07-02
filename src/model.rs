#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum SubjectCondition {
    AskExplicitly,
    Allowed,
    AllowedIfNonCommercialOrPayedNonCommercial,
    AllowedIfNonCommercial,
    Disallowed,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum GenericPermission {
    Allowed,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ReLendingRightsToOtherPartyCondition {
    AllowedToAnyone,
    AllowedToSpecified,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum SensitiveUsagePermission {
    Allowed,
    AllowedAndRequestedZoning,
    DisallowedButPrivate,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum PoliticsAndReligionUsagePermission {
    Allowed,
    DisallowedButPrivate,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum OtherPartyRestructurePermission {
    Allowed,
    AllowedBetweenUsers,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum RedistributionPermission {
    Allowed,
    AllowedIfFree,
    AllowedIfBindToThisTerms,
    AllowedIfFreeAndBindToThisTerms,
    AllowedBetweenUsers,
    AllowedBetweenUsersIfFree,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum EmbeddedPermission {
    Allowed,
    AllowedIfNoRiskOfConfusion,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum NijiPermission {
    Allowed,
    AllowedIfNonCommercialOrPaidNonCommercial,
    AllowedIfNonCommercial,
    AllowedIfPrivate,
    Disallowed,
    NotApplicable,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum CompletelyNewNijiRedistributePermission {
    Allowed,
    AllowedIfNonCommercialOrPaidNonCommercial,
    AllowedIfNonCommercial,
    Disallowed,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum CreditCondition {
    Required,
    NotRequiredButPreferred,
    NotRequired,
    AskExplicitly,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Vn3License {
    pub subject_personal: SubjectCondition,
    pub subject_houjin: SubjectCondition,
    pub usage_social: GenericPermission,
    pub usage_online: GenericPermission,
    pub usage_retain_permission_to_other_party: ReLendingRightsToOtherPartyCondition,
    pub sensitive_sexual: SensitiveUsagePermission,
    pub sensitive_violence: SensitiveUsagePermission,
    pub sensitive_seiji_and_syukyou: PoliticsAndReligionUsagePermission,
    pub restructure_reformat_and_decimate: GenericPermission,
    pub restructure_modify_remains_same_data: GenericPermission,
    pub restructure_and_export_to_other_data: GenericPermission,
    pub ask_restructure_and_lent_to_other_party: OtherPartyRestructurePermission,
    pub redistribute_unmodified: RedistributionPermission,
    pub redistribute_modified: RedistributionPermission,
    pub embed_video: EmbeddedPermission,
    pub embed_publish: EmbeddedPermission,
    pub embed_goods: EmbeddedPermission,
    pub embed_software: EmbeddedPermission,
    pub niji_make_derived: NijiPermission,
    pub niji_make_original_in_format: NijiPermission,
    pub niji_completely_new: CompletelyNewNijiRedistributePermission,
    pub credit: CreditCondition,
    pub transfer_rights_and_obligations_to_other_party: GenericPermission,
    pub special_notes: String,
    pub first_party_name: String,
    pub first_party_email_address: String,
    pub first_party_twitter: String,
    pub first_party_website: String,
    pub first_party_credit: String,
    pub recommended_hashtags: String,
}
