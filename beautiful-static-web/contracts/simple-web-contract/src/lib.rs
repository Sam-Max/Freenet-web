use freenet_stdlib::prelude::*;

pub struct WebContract;

#[contract]
impl ContractInterface for WebContract {
    fn validate_state(
        _parameters: Parameters<'static>,
        state: State<'static>,
        _related: RelatedContracts<'static>,
    ) -> Result<ValidateResult, ContractError> {
        // En un caso real, validaríamos que el estado sea un archivo tar.xz válido
        if state.len() < 10 {
            return Ok(ValidateResult::Invalid);
        }
        Ok(ValidateResult::Valid)
    }

    fn update_state(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        data: Vec<UpdateData<'static>>,
    ) -> Result<UpdateModification<'static>, ContractError> {
        if let Some(UpdateData::State(new_state)) = data.first() {
             Ok(UpdateModification::valid(new_state.clone()))
        } else {
            Err(ContractError::InvalidUpdate)
        }
    }

    fn summarize_state(
        _parameters: Parameters<'static>,
        state: State<'static>,
    ) -> Result<StateSummary<'static>, ContractError> {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&state);
        let hash = hasher.finalize();
        Ok(StateSummary::from(hash.to_vec()))
    }

    fn get_state_delta(
        _parameters: Parameters<'static>,
        state: State<'static>,
        _summary: StateSummary<'static>,
    ) -> Result<StateDelta<'static>, ContractError> {
        Ok(StateDelta::from(state.into_bytes()))
    }
}
