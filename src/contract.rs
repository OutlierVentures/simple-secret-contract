use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use crate::msg::{TotalResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        total: msg.total,
        owner: env.message.sender,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddSquared { amount } => try_addsquared(deps, env, amount),
        HandleMsg::Reset { total } => try_reset(deps, env, total),
    }
}

pub fn try_addsquared<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    amount: i32
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state.total += amount * amount;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    total: i32,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        if env.message.sender != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        state.total = total;
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTotal {} => to_binary(&query_total(deps)?),
    }
}

fn query_total<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<TotalResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(TotalResponse { total: state.total })
}

