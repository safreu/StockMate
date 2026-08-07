use crate::modules::households::{
    domain::{Household, HouseholdKind, HouseholdMember, HouseholdRole},
    ports::HouseholdRepositoryError,
};

pub(super) fn validate_aggregate(
    household: &Household,
    owner: &HouseholdMember,
) -> Result<(), HouseholdRepositoryError> {
    if owner.household_id() != household.id() {
        return Err(HouseholdRepositoryError::InvalidAggregate);
    }

    if owner.role() != HouseholdRole::Owner {
        return Err(HouseholdRepositoryError::InvalidAggregate);
    }

    if household.kind() == HouseholdKind::Personal
        && household.personal_owner_id() != Some(owner.user_id())
    {
        return Err(HouseholdRepositoryError::InvalidAggregate);
    };

    Ok(())
}
