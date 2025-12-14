#[macro_use]
extern crate napi_derive;

/// Verify a Thai Citizen Card ID.
#[napi]
pub fn verify_id(id: String) -> bool {
    konjingjing::verify_id(&id)
}
