use cosmwasm_std::{entry_point, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use cosmwasm_std::StdError;
use schemars::JsonSchema;
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, SwapperError> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SifchainMsg>, SwapperError> {
    match msg {
        ExecuteMsg::Swap { amount } => {
            let swap_msg = SifchainMsg::Swap {
                sent_asset: "rowan".to_string(),
                received_asset: "ceth".to_string(),
                sent_amount: amount.to_string(),
                min_received_amount: "0".to_string(),
            };

            Ok(Response::new()
                .add_attribute("action", "swap")
                .add_message(swap_msg))
        }
        ExecuteMsg::AddLiquidity {} => {
            let add_liquidity_msg = SifchainMsg::AddLiquidity {
                external_asset: "ceth".to_string(),
                native_asset_amount: "100".to_string(),
                external_asset_amount: "50".to_string(),
            };

            Ok(Response::new()
                .add_attribute("action", "add_liquidity")
                .add_message(add_liquidity_msg))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] //JsonSchema removed
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap { amount: u32 },
    AddLiquidity {},
}

#[derive(Error, Debug)]
pub enum SwapperError {
    #[error("{0}")]
    Std(#[from] StdError),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SifchainMsg {
    Swap {
        sent_asset: String,
        received_asset: String,
        sent_amount: String,
        min_received_amount: String,
    },
    AddLiquidity {
        external_asset: String,
        native_asset_amount: String,
        external_asset_amount: String,
    },
}

impl cosmwasm_std::CustomMsg for SifchainMsg {}

impl From<SifchainMsg> for CosmosMsg<SifchainMsg> {
    fn from(original: SifchainMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}
