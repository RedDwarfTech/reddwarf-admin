use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EditDomainRequest {
    pub domainName: String,
    pub domainUrl: String,
    pub id: i64
}