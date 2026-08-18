use chrono::{DateTime, Utc};

use crate::modules::{
    devices::domain::{DeviceId, DeviceKind, device_name::DeviceName},
    households::domain::HouseholdId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    id: DeviceId,
    household_id: HouseholdId,
    name: DeviceName,
    kind: DeviceKind,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Device {
    pub fn new(
        id: DeviceId,
        household_id: HouseholdId,
        name: DeviceName,
        kind: DeviceKind,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            household_id,
            name,
            kind,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> DeviceId {
        self.id
    }

    pub fn household_id(&self) -> HouseholdId {
        self.household_id
    }

    pub fn name(&self) -> &DeviceName {
        &self.name
    }

    pub fn kind(&self) -> DeviceKind {
        self.kind
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
