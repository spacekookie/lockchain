use lockchain::traits::{AutoEncoder, Body, Vault};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct ApiState<B, V>
where
    B: Body,
    V: Vault<B>,
{
    vaults: HashMap<String, Option<V>>,
    _phantom: PhantomData<B>,
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
    fn from(me: ApiState<B, V>) -> SerializedState {
        SerializedState {
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
    fn from(me: SerializedState) -> ApiState<B, V> {
        ApiState {
            vaults: me.vaults.into_iter().fold(
                HashMap::new(),
                |mut acc: HashMap<String, Option<V>>, k| {
                    acc.insert(k, None);
                    acc
                },
            ),
            _phantom: PhantomData,
        }
    }
}
