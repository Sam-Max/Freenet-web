use freenet_stdlib::prelude::*;

struct WebDelegate;

#[delegate]
impl DelegateInterface for WebDelegate {
    fn process(
        _parameters: Parameters<'static>,
        _attested: Option<&'static [u8]>,
        _message: InboundDelegateMsg,
    ) -> Result<Vec<OutboundDelegateMsg>, DelegateError> {
        Ok(vec![])
    }
}
