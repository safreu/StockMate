use backend::modules::{
    accounts::{
        adapters::PostgresUserRepository,
        domain::{
            DisplayName, Email, PasswordHash, Session, SessionId, SessionTokenHash, User, UserId,
        },
        ports::UserRepository,
    },
    households::{
        adapters::PostgresHouseholdRepository,
        domain::{
            Household, HouseholdId, HouseholdKind, HouseholdMember, HouseholdName, HouseholdRole,
        },
        ports::HouseholdRepository,
    },
    inventory::domain::{Category, CategoryId, CategoryName},
};
use chrono::{Duration, SubsecRound, TimeZone, Utc};

pub fn test_session(user_id: &UserId, token_hash: &str) -> Session {
    let created_at = Utc
        .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
        .single()
        .expect("Timestamp should be valid");

    let expires_at = created_at + Duration::hours(1);

    Session::new(
        SessionId::new(),
        *user_id,
        SessionTokenHash::from_encoded(token_hash).expect("Test token hash should be valid"),
        expires_at,
        created_at,
    )
    .expect("Test session should be valid")
}

pub fn test_user(email: &str) -> User {
    User::new(
        UserId::new(),
        Email::parse(email)
            .expect("Test email should be valid"),
        DisplayName::parse("valid name").expect("Test display name should be valid"),
        PasswordHash::from_encoded("$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$7Qdih1MuhjZehB6Svms5vcBhkM4A5f7QWwD4iM4R+AE")
            .expect("Test password hash should be valid"),
    )
}

pub async fn insert_test_user(repository: &PostgresUserRepository) -> User {
    let user = test_user("valid@email.com");

    repository
        .insert(&user)
        .await
        .expect("Test user should be insertable");

    user
}

pub fn test_category(household_id: HouseholdId, name: &str) -> Category {
    Category::new(
        CategoryId::new(),
        household_id,
        CategoryName::parse(name).expect("Category name should be valid"),
        Utc::now().trunc_subsecs(6),
        Utc::now().trunc_subsecs(6),
    )
}

pub async fn insert_test_user_with_email(repository: &PostgresUserRepository, email: &str) -> User {
    let user = test_user(email);

    repository
        .insert(&user)
        .await
        .expect("Test user should be insertable");

    user
}

pub fn create_owned_household(
    user_id: UserId,
    kind: HouseholdKind,
) -> (Household, HouseholdMember) {
    let household_id = HouseholdId::new();
    let now = Utc::now().trunc_subsecs(6);

    let personal_owner_id = match kind {
        HouseholdKind::Personal => Some(user_id),
        HouseholdKind::Shared => None,
    };

    let household = Household::new(
        household_id,
        HouseholdName::parse("Test household").expect("Test household name should be valid"),
        kind,
        personal_owner_id,
        now,
        now,
    )
    .expect("Test household should be valid");

    let owner = HouseholdMember::new(household_id, user_id, HouseholdRole::Owner, now);

    (household, owner)
}

pub async fn insert_owned_household(
    repository: &PostgresHouseholdRepository,
    user_id: UserId,
    kind: HouseholdKind,
) -> (Household, HouseholdMember) {
    let (household, owner) = create_owned_household(user_id, kind);

    repository
        .create_with_owner(&household, &owner)
        .await
        .expect("Test household should be insertable");

    (household, owner)
}
