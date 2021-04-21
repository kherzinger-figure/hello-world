use cosmwasm_std::{attr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use provwasm_std::{bind_name, NameBinding, ProvenanceMsg};

use crate::contract_info::{get_contract_info, set_contract_info, ContractInfo};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{get_message_storage, get_message_storage_read, MessageState};

// smart contract initialization entrypoint
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    if msg.bind_name.is_empty() {
        return Err(ContractError::MissingField {
            field: "bind_name".into(),
        });
    }
    if msg.contract_name.is_empty() {
        return Err(ContractError::MissingField {
            field: "contract_name".into(),
        });
    }

    // set contract info
    let contract_info = ContractInfo::new(info.sender, msg.bind_name, msg.contract_name);
    set_contract_info(deps.storage, &contract_info)?;

    // create name binding provenance message
    let bind_name_msg = bind_name(
        contract_info.bind_name,
        env.contract.address,
        NameBinding::Restricted,
    )?;

    // build response
    Ok(Response {
        submessages: vec![],
        messages: vec![bind_name_msg],
        attributes: vec![
            attr(
                "contract_info",
                format!("{:?}", get_contract_info(deps.storage)?),
            ),
            attr("action", "init"),
        ],
        data: None,
    })
}

// smart contract execute entrypoint
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    match msg {
        ExecuteMsg::SayHello { id } => store_message(deps, info, id, "Hello world.".to_string()),
        ExecuteMsg::SayGoodbye { id } => {
            store_message(deps, info, id, "Goodbye, cruel world.".to_string())
        }
        ExecuteMsg::SaySomethingElse { id, message } => store_message(deps, info, id, message),
    }
}

fn store_message(
    deps: DepsMut,
    _info: MessageInfo,
    id: String,
    msg: String,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    if msg.is_empty() {
        return Err(ContractError::MissingField {
            field: "msg".into(),
        });
    }

    if id.is_empty() {
        return Err(ContractError::MissingField { field: "id".into() });
    }

    let mut msg_storage = get_message_storage(deps.storage);

    let message_state = MessageState { id, message: msg };

    msg_storage.save(&message_state.id.as_bytes(), &message_state)?;

    Ok(Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![attr("action", "store_message")],
        data: Some(to_binary(&message_state)?),
    })
}

// smart contract query entrypoint
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { id } => {
            let msg_storage_read = get_message_storage_read(deps.storage);
            return to_binary(&msg_storage_read.load(id.as_bytes())?);
        }
        QueryMsg::GetContractInfo {} => to_binary(&get_contract_info(deps.storage)?),
    }
}
