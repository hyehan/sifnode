use cosmwasm_std::{entry_point, to_binary, CosmosMsg, CustomQuery};
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo};
use cosmwasm_std::{Response, QueryResponse, StdResult, StdError};

use schemars::JsonSchema;
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum SwapperError {
    #[error("{0}")]
    Std(#[from] StdError),
}

/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Instantiate 
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] //JsonSchema removed
pub struct InstantiateMsg {}


#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, SwapperError> {
    Ok(Response::default())
}

/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Execute
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap { amount: u32 },
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
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
       
    }
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
}

impl cosmwasm_std::CustomMsg for SifchainMsg {}

impl From<SifchainMsg> for CosmosMsg<SifchainMsg> {
    fn from(original: SifchainMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Query
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Pool { external_asset: String },
}

#[entry_point]
pub fn query(
    deps: Deps<SifchainQuery>,
     _env: Env,
      msg: QueryMsg,
) -> StdResult<QueryResponse> {

    match msg {
        QueryMsg::Pool { external_asset} => to_binary(&query_pool(deps, external_asset)?),
    }
}

fn query_pool(deps: Deps<SifchainQuery>, _external_asset: String) -> StdResult<SifchainResponse> {
    let req = SifchainQuery::Ping { }.into();
    let response: SifchainResponse = deps.querier.query(&req)?;
    Ok(response)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SifchainQuery {
    Ping {},
}

impl CustomQuery for SifchainQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SifchainResponse {
    pub msg: String,
}