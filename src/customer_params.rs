#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerParams {
    pub application_time: String,
    pub salary: i32,
    pub occupation: String,
    pub age: i8
}