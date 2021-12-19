use rocket::response::content;

#[get("/v1/dashboard/overview")]
pub fn overview() -> content::Json<String> {
    return content::Json("response_json".parse().unwrap());
}





