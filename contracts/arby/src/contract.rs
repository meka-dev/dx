use std::str::from_utf8;

use astroport::{
    asset::{Asset, AssetInfo},
    pair::{Cw20HookMsg, ExecuteMsg as AstroExecuteMsg, QueryMsg, SimulationResponse},
};
#[allow(unused_imports)]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins,
    to_binary,
    Addr,
    BankMsg,
    Coin,
    CosmosMsg,
    DepsMut,
    Env,
    MessageInfo,
    Reply,
    ReplyOn,
    Response,
    SubMsg,
    Uint128,
    WasmMsg,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;
use cw_storage_plus::{Item, Map};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg},
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:mtek-arby";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const POST_BALANCE: Item<'_, Uint128> = Item::new("post_balance");
const OWNER: Item<'_, Addr> = Item::new("owner");
const SPRINTERS: Map<'_, Addr, ()> = Map::new("sprinters");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
    for s in msg.sprinters {
        SPRINTERS.save(deps.storage, s, &())?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetSprinters { sprinters } => set_sprinters(deps, info, sprinters),
        ExecuteMsg::Sprint {
            input,
            min_out,
            path,
        } => sprint(deps, info, env, input, min_out, path),
        ExecuteMsg::Withdraw { asset, amount, to } => withdraw(deps, info, asset, amount, to),
    }
}

const POST_BALANCE_REPLY_ID: u64 = 1;
const JUNCTION_REPLY_ID: u64 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<'_>, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        POST_BALANCE_REPLY_ID => assert_post_balance(deps, env, msg),
        JUNCTION_REPLY_ID => Ok(Response::new()),
        _ => Err(ContractError::Unsupported {}),
    }
}

pub fn assert_post_balance(
    deps: DepsMut<'_>,
    env: Env,
    _msg: Reply,
) -> Result<Response, ContractError> {
    let bal = deps.querier.query_balance(env.contract.address, "uusd")?;
    if bal.amount < POST_BALANCE.load(deps.storage)? {
        return Err(ContractError::InsufficientPostBalance {});
    }

    Ok(Response::new())
}

pub fn set_sprinters(
    deps: DepsMut<'_>,
    info: MessageInfo,
    sprinters: Vec<Addr>,
) -> Result<Response, ContractError> {
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let old: Vec<_> = SPRINTERS
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    for k in old {
        SPRINTERS.remove(
            deps.storage,
            Addr::unchecked(from_utf8(&k).map_err(|_| ContractError::Unsupported {})?),
        )
    }

    for s in sprinters {
        SPRINTERS.save(deps.storage, s, &())?;
    }

    Ok(Response::new())
}

pub fn sprint(
    deps: DepsMut<'_>,
    info: MessageInfo,
    env: Env,
    input: Uint128,
    min_out: Uint128,
    path: Vec<(Addr, AssetInfo)>,
) -> Result<Response, ContractError> {
    if path.len() < 2 {
        return Err(ContractError::PathTooShort {});
    }
    if !SPRINTERS.has(deps.storage, info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let bal = match path.get(0) {
        Some((_, AssetInfo::NativeToken { denom })) if denom == "uust" => {
            deps.querier.query_balance(env.contract.address, denom)?
        },
        _ => return Err(ContractError::UnsupportedStartAsset {}),
    };

    POST_BALANCE.save(deps.storage, &(min_out + bal.amount))?;

    let mut msgs: Vec<SubMsg> = vec![];
    let mut input = input;
    for (addr, ass) in path.iter().take(path.len() - 1) {
        let rq = QueryMsg::Simulation {
            offer_asset: Asset {
                info: (*ass).clone(),
                amount: input,
            },
        };
        // this query is expensive
        let sim: SimulationResponse = deps.querier.query_wasm_smart(addr, &rq)?;
        msgs.push(SubMsg {
            id: JUNCTION_REPLY_ID,
            msg: mk_msg(addr, ass, input)?,
            gas_limit: None,
            reply_on: ReplyOn::Never,
        });
        input = sim.return_amount;
    }

    if let Some((addr, ass)) = path.iter().last() {
        msgs.push(SubMsg {
            id: POST_BALANCE_REPLY_ID,
            msg: mk_msg(addr, ass, input)?,
            gas_limit: None,
            reply_on: ReplyOn::Success,
        });
    } else {
        return Err(ContractError::Unsupported {});
    }

    Ok(Response::new().add_submessages(msgs))
}

fn mk_msg(addr: &Addr, ass: &AssetInfo, amount: Uint128) -> Result<CosmosMsg, ContractError> {
    match ass {
        AssetInfo::Token { contract_addr } => {
            let swp = Cw20HookMsg::Swap {
                belief_price: None,
                max_spread: None,
                to: None,
            };
            Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Send {
                    amount,
                    msg: to_binary(&swp)?,
                    contract: addr.to_string(),
                })?,
                funds: vec![],
            }))
        },
        AssetInfo::NativeToken { denom } => Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: addr.to_string(),
            msg: to_binary(&AstroExecuteMsg::Swap {
                offer_asset: Asset {
                    info: AssetInfo::NativeToken {
                        denom: denom.clone(),
                    },
                    amount,
                },
                belief_price: None,
                max_spread: None,
                to: None,
            })?,
            funds: vec![Coin {
                amount,
                denom: denom.clone(),
            }],
        })),
    }
}

pub fn withdraw(
    deps: DepsMut<'_>,
    info: MessageInfo,
    asset: AssetInfo,
    amount: Uint128,
    to: Addr,
) -> Result<Response, ContractError> {
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let msg: CosmosMsg = match asset {
        AssetInfo::Token { contract_addr } => CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: to.to_string(),
                amount,
            })?,
            funds: vec![],
        }),
        AssetInfo::NativeToken { denom } => BankMsg::Send {
            to_address: to.to_string(),
            amount: coins(amount.into(), denom),
        }
        .into(),
    };

    Ok(Response::new().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut<'_>, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
        ContractResult,
        SubMsgExecutionResponse,
    };

    use super::*;

    #[test]
    fn proper_initialization() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        assert_eq!(
            Ok(()),
            SPRINTERS.load(&deps.storage, Addr::unchecked("foo"))
        );
    }

    #[test]
    fn set_sprinters() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SetSprinters {
            sprinters: vec![Addr::unchecked("baz"), Addr::unchecked("bar")],
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_ne!(
            Ok(()),
            SPRINTERS.load(&deps.storage, Addr::unchecked("foo"))
        );

        assert_eq!(
            Ok(()),
            SPRINTERS.load(&deps.storage, Addr::unchecked("baz"))
        );
        assert_eq!(
            Ok(()),
            SPRINTERS.load(&deps.storage, Addr::unchecked("bar"))
        );
    }

    #[test]
    fn prevent_non_owner_set_sprinters() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("other", &[]);
        let msg = ExecuteMsg::SetSprinters {
            sprinters: vec![Addr::unchecked("baz"), Addr::unchecked("bar")],
        };
        assert_eq!(
            Err(ContractError::Unauthorized {}),
            execute(deps.as_mut(), mock_env(), info, msg)
        );
    }

    #[test]
    fn assert_post_balance_success() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(20, "uusd"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        POST_BALANCE
            .save(&mut deps.storage, &Uint128::from(10u128))
            .unwrap();

        let msg = Reply {
            id: POST_BALANCE_REPLY_ID,
            result: ContractResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: None,
            }),
        };
        let res = reply(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(Response::new(), res);
    }

    #[test]
    fn assert_post_balance_failure() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(20, "uusd"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        POST_BALANCE
            .save(&mut deps.storage, &Uint128::from(21u128))
            .unwrap();

        let msg = Reply {
            id: POST_BALANCE_REPLY_ID,
            result: ContractResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: None,
            }),
        };
        assert_eq!(
            Err(ContractError::InsufficientPostBalance {}),
            reply(deps.as_mut(), mock_env(), msg),
        );
    }

    #[test]
    fn owner_withdraw() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = ExecuteMsg::Withdraw {
            asset: AssetInfo::NativeToken {
                denom: "token".to_string(),
            },
            amount: Uint128::from(2u128),
            to: Addr::unchecked("bar"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(
            vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "bar".to_owned(),
                amount: coins(2, "token")
            }))],
            res.messages,
        );
    }

    #[test]
    fn prevent_non_owner_withdraw() {
        // let mut deps = mock_dependencies_with_balances(vec![()]&coins(2, "token"));
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {
            sprinters: vec![Addr::unchecked("foo")],
        };
        let info = mock_info("owner", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let info = mock_info("other", &[]);
        let msg = ExecuteMsg::Withdraw {
            asset: AssetInfo::NativeToken {
                denom: "token".to_string(),
            },
            amount: Uint128::from(2u128),
            to: Addr::unchecked("bar"),
        };
        assert_eq!(
            Err(ContractError::Unauthorized {}),
            execute(deps.as_mut(), mock_env(), info, msg)
        );
    }
}
