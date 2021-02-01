use super::Rpc;
use crate::types::{BlockHeader, SyncInfoSub, H256, U128};
use std::marker::PhantomData;

pub struct SubscriptionRequest<T> {
    pub(crate) rpc: Rpc<U128>,
    result_type: PhantomData<T>,
}

pub fn eth_subscribe_new_heads() -> SubscriptionRequest<BlockHeader> {
    let mut rpc = Rpc::new("eth_subscribe");
    rpc.add_param("newHeads");
    SubscriptionRequest {
        rpc,
        result_type: PhantomData,
    }
}

pub fn eth_subscribe_new_pending_transactions() -> SubscriptionRequest<H256> {
    let mut rpc = Rpc::new("eth_subscribe");
    rpc.add_param("newPendingTransactions");
    SubscriptionRequest {
        rpc,
        result_type: PhantomData,
    }
}

pub fn eth_subscribe_syncing() -> SubscriptionRequest<SyncInfoSub> {
    let mut rpc = Rpc::new("eth_subscribe");
    rpc.add_param("syncing");
    SubscriptionRequest {
        rpc,
        result_type: PhantomData,
    }
}

pub fn eth_unsubscribe(sub_id: U128) -> Rpc<bool> {
    let mut rpc = Rpc::new("eth_unsubscribe");
    rpc.add_param(sub_id);
    rpc
}
