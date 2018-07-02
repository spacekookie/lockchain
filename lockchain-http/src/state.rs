use lockchain::traits::{AutoEncoder, Body, FileIO, Vault};

use std::collections::{HashMap, HashSet};
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
    #[doc(hidden)]
    pub vaults: HashMap<String, Option<V>>,
    #[doc(hidden)]
    pub _phantom: PhantomData<B>,
    #[doc(hidden)]
    pub tokens: HashSet<String>,
    /// Signal if the API handlers are allowed outside their working dir
    pub bound_scope: bool,
    /// Provide a working directory
    pub working_dir: PathBuf,
    /// Completely disabe administrative actions
    pub administrative: bool,
}

impl<B, V> ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    /// Load an existing API state from an encoded string
    pub fn load(encoded: &str) -> Option<Self> {
        SerializedState::decode(encoded).ok().map(|s| s.into())
    }

    /// Store an in-memory API state to an encoded string
    pub fn store(&self) -> String {
        SerializedState::from(self).encode().ok().unwrap()
    }

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

    pub fn get_vault(&mut self, name: &str) -> Option<&mut V> {
        self.vaults.get_mut(name)?.as_mut()
    }
}

impl<B, V> Default for ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    #[allow(unconditional_recursion)]
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            bound_scope: true,
            vaults: HashMap::new(),
            tokens: HashSet::new(),
            administrative: false,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedState {
    vaults: Vec<String>,
}

impl AutoEncoder for SerializedState {}
impl FileIO for SerializedState {}

/// Implements the transform from in-memory to on-disk
impl<'state, B, V> From<&'state ApiState<B, V>> for SerializedState
where
    B: Body,
    V: Vault<B>,
{
    fn from(me: &'state ApiState<B, V>) -> Self {
        Self {
            vaults: me.vaults.iter().map(|(k, _)| k.clone()).collect(),
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
            vaults: me.vaults.into_iter().map(|k| (k, None)).collect(),
            _phantom: PhantomData,
            ..Default::default()
        }
    }
}
