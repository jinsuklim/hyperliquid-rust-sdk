use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{order::OrderRequest, ClientOrderRequest};

#[derive(Debug)]
pub struct ClientModifyRequest {
    pub oid: Uuid,
    pub order: ClientOrderRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifyRequest {
    pub oid: String,
    pub order: OrderRequest,
}
