use crate::*;

pub(crate) fn is_valid(email: &ParsedEmail) -> bool {
    true
}

pub(crate) async fn send_to_coordinator(email: &str) -> Result<()> {
    Ok(())
}

pub(crate) async fn response_from_modal() -> Result<String> {
    Ok(String::new())
}

pub(crate) async fn send_response() -> Result<()> {
    Ok(())
}
