use lockchain::users::Role;

/// Fields provided when creating a new vault
#[derive(Serialize, Deserialize)]
pub struct VaultCreate {
    pub name: String,
    pub location: String,
    pub token: String,
}

/// Fields provided when deleting a vault
#[derive(Serialize, Deserialize)]
pub struct VaultDelete {
    pub name: String,
    pub location: String,
    pub token: String,
}

/// Add a vault to the API scope
#[derive(Serialize, Deserialize)]
pub struct ScopeVault {
    pub name: String,
    pub location: String,
    pub token: String,
}

/// Remove a vault from the API scope
#[derive(Serialize, Deserialize)]
pub struct UnscopeVault {
    pub name: String,
    pub location: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateVault {
    pub name: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRecord {
    pub vault: String,
    pub token: String,
}

/// Query to get a record
#[derive(Serialize, Deserialize)]
pub struct GetRecord {
    pub name: String,
    pub range: Option<(u32, u32)>,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRecord {
    pub vault: String,
    pub record: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteRecord {
    pub vault: String,
    pub record: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Authenticate {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Deauthenticate {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub requested_role: Option<Role>,
}
