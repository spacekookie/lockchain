use lockchain::traits::{AutoEncoder, Body, Vault};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::PathBuf;

/// An in-memory API state object which is delegated to all handlers
///
/// This mechanism serves two purposes
///
/// 1. Configuration of the API, beyond simple paramters provided to
/// the server_start call
/// 2. Buffering and pre-loading of certain vault components that need
/// to be accessed via the handlers
///
/// It provides some simple query functions for handlers to work on,
/// as well as expose raw configuration fields to be written
///
/// ```
/// let state: ApiState<B, V> = ApiState {
///     bound_scope: false,
///     working_dir: ".".into(),
///     ..
/// };
/// ```
///
/// (Replace `B` and `V` with your generics 🙂)
pub struct ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    vaults: HashMap<String, Option<V>>,
    _phantom: PhantomData<B>,

    /// Signal if the API handlers are allowed outside their working dir
    pub bound_scope: bool,
    /// Provide a working directory
    pub working_dir: PathBuf,
}

impl<B, V> ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    /// Return a list of string slices for each vault in scope
    pub fn vaults(&self) -> Vec<&str> {
        self.vaults.iter().map(|(k, _)| k.as_str()).collect()
    }
    /// Simply return the number of known vaults
    pub fn count(&self) -> usize {
        self.vaults.len()
    }

    pub fn add_vault(&mut self, name: &str, vault: V) {
        self.vaults.insert(name.into(), Some(vault));
    }
}

impl<B, V> Default for ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    fn default() -> Self {
        Self {
            vaults: Default::default(),
            _phantom: PhantomData,
            bound_scope: true,
            working_dir: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedState {
    vaults: Vec<String>,
}

impl AutoEncoder for SerializedState {}

/// Implements the transform from in-memory to on-disk
impl<B, V> From<ApiState<B, V>> for SerializedState
where
    B: Body,
    V: Vault<B>,
{
    fn from(me: ApiState<B, V>) -> Self {
        Self {
            vaults: me
                .vaults
                .into_iter()
                .fold(Vec::new(), |mut acc: Vec<String>, (k, v)| {
                    acc.push(k);
                    acc
                }),
        }
    }
}

/// Implements the transform from on-disk to in-memory
impl<B, V> From<SerializedState> for ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    fn from(me: SerializedState) -> Self {
        Self {
            vaults: me.vaults.into_iter().fold(
                HashMap::new(),
                |mut acc: HashMap<String, Option<V>>, k| {
                    acc.insert(k, None);
                    acc
                },
            ),
            _phantom: PhantomData,
            ..Default::default()
        }
    }
}
