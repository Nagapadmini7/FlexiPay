use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg>
    where
        T: Serialize + ?Sized,
    {
        let binary_msg = to_json_binary(&msg)?;

        let execution_result = WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: binary_msg,
            funds: vec![],
        };

        Ok(execution_result.into())
    }
}
