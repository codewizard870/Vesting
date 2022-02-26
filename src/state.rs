use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};

//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token_addr: Addr,
	pub start_time: Uint128,
}

//------------Vesting parameter---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct VestingParameter{																																																																																																																																																																																																																																																																																																			
	pub soon: Uint128,
	pub after: Uint128,
	pub period: Uint128
}

// pub const VEST_PARAM: Map<String, VestingParameter> = Map::new("vestingparameter");

//-------------Token holder-------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet_address: Addr, //investor wallet address
	pub total_amount: Uint128, //WFD token total amount that the investor buys.
	pub released_amount: Uint128, //released WFD token amount of totalAmount
	pub pending_amount: Uint128, //token amount that investor can claim 
}

// pub const SEED_USERS: Item<Vec<UserInfo>> = Item::new("seedusers");
// pub const PRESALE_USERS: Item<Vec<UserInfo>> = Item::new("presaleusers");
// pub const IDO_USERS: Item<Vec<UserInfo>> = Item::new("idousers");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectInfo{
	pub project_id: Uint128,
	pub config: Config,
	pub vest_param: HashMap<String, VestingParameter>,
	pub seed_users: Vec<UserInfo>,
	pub presale_users: Vec<UserInfo>,
	pub ido_users: Vec<UserInfo>,
}

pub const OWNER: Item<Addr> = Item::new("owner");

pub const PROJECT_SEQ: Item<Uint128> = Item::new("prj_seq");
pub const PROJECT_INFOS:Map<U128Key, ProjectInfo> = Map::new("project_infos");

pub fn save_projectinfo(deps: DepsMut, _prj: &mut ProjectInfo) 
    -> StdResult<()> 
{
    // increment id if exists, or return 1
    let id = PROJECT_SEQ.load(deps.storage)?;
    let id = id.checked_add(Uint128::new(1))?;
    PROJECT_SEQ.save(deps.storage, &id)?;

    _prj.project_id = id;
    PROJECT_INFOS.save(deps.storage, id.u128().into(), &_prj)
}
