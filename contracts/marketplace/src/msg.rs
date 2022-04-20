use crate::state::{Ask, Bid, TokenId};
use cosmwasm_std::{Addr, Coin, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// List an NFT on the marketplace by creating a new ask
    SetAsk {
        collection: String,
        token_id: TokenId,
        price: Coin,
        funds_recipient: Option<String>,
        expires: Timestamp,
    },
    /// Remove an existing ask from the marketplace
    RemoveAsk {
        collection: String,
        token_id: TokenId,
    },
    /// Admin operation to change the active state of an ask when an NFT is transferred
    UpdateAskState {
        collection: String,
        token_id: TokenId,
        active: bool,
    },
    /// Update the price of an existing ask
    UpdateAsk {
        collection: String,
        token_id: TokenId,
        price: Coin,
    },
    /// Place a bid on an existing ask
    SetBid {
        collection: String,
        token_id: TokenId,
        expires: Timestamp,
    },
    /// Remove an existing bid from an ask
    RemoveBid {
        collection: String,
        token_id: TokenId,
    },
    /// Accept a bid on an existing ask
    AcceptBid {
        collection: String,
        token_id: TokenId,
        bidder: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the current ask for specific NFT
    /// Return type: `CurrentAskResponse`
    CurrentAsk {
        collection: String,
        token_id: TokenId,
    },
    /// Get all asks for a collection
    /// Return type: `AsksResponse`
    Asks {
        collection: String,
        start_after: Option<TokenId>,
        limit: Option<u32>,
    },
    /// Count of all asks
    /// Return type: `AskCountResponse`
    AskCount { collection: String },
    /// Get all asks by seller
    /// Return type: `AsksResponse`
    AsksBySeller { seller: String },
    /// List of collections that have asks on them
    /// Return type: `CollectionsResponse`
    ListedCollections {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Get data for a specific bid
    /// Return type: `BidResponse`
    Bid {
        collection: String,
        token_id: TokenId,
        bidder: String,
    },
    /// Get all bids by a bidder
    /// Return type: `BidsResponse`
    BidsByBidder { bidder: String },
    /// Get all bids for a specific NFT
    /// Return type: `BidsResponse`
    Bids {
        collection: String,
        token_id: TokenId,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CurrentAskResponse {
    pub ask: Option<Ask>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AsksResponse {
    pub asks: Vec<Ask>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AskCountResponse {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionsResponse {
    pub collections: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidResponse {
    pub bid: Option<Bid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidsResponse {
    pub bids: Vec<Bid>,
}
