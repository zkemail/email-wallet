use crate::*;

pub(crate) enum EmailStatus {
    Unchecked,
    Checked,
    Executed,
}

pub(crate) fn check_if_valid(email: &str) -> bool {
    todo!()
}

pub(crate) async fn send_to_coordinator(email: &str) -> Result<()> {
    todo!()
}

pub(crate) async fn response_from_modal() -> Result<String> {
    todo!()
}

pub(crate) async fn send_response() -> Result<()> {
    todo!()
}
