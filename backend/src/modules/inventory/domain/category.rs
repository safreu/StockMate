use chrono::{DateTime, Utc};

use crate::modules::{
    households::domain::HouseholdId,
    inventory::domain::{CategoryId, CategoryName},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    id: CategoryId,
    household_id: HouseholdId,
    name: CategoryName,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Category {
    pub fn new(
        id: CategoryId,
        household_id: HouseholdId,
        name: CategoryName,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            household_id,
            name,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> CategoryId {
        self.id
    }

    pub fn household_id(&self) -> HouseholdId {
        self.household_id
    }

    pub fn name(&self) -> &CategoryName {
        &self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn rename(&mut self, name: CategoryName, now: DateTime<Utc>) {
        self.name = name;
        self.updated_at = now;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn category_can_be_renamed() {
        let now = Utc::now();

        let mut category = Category::new(
            CategoryId::new(),
            HouseholdId::new(),
            CategoryName::parse("Food").expect("Name should be valid"),
            now,
            now,
        );

        let updated_at = Utc::now();

        category.rename(
            CategoryName::parse("Drinks").expect("Name should be valid"),
            updated_at,
        );

        assert_eq!(category.name().as_str(), "Drinks");
        assert_eq!(category.updated_at(), updated_at)
    }
}
