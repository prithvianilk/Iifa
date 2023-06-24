use chrono::{Local, TimeZone};
use crate::customer_params::CustomerParams;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum Predicate {
    AppliedBeforeDeadline{application_deadline: String},
    SalaryAbove{min_salary: i32},
    AgeAbove{min_age: i8},
    InGoodOccupations{occupations: Vec<String>},
    DEFAULT
}

impl Default for Predicate {
    fn default() -> Self { Predicate::DEFAULT }
}

pub fn evaluate(predicate: &Predicate, customer_params: &CustomerParams) -> bool {
    match predicate {
        Predicate::AppliedBeforeDeadline{application_deadline} => is_date_before(&customer_params.application_time, application_deadline),
        Predicate::SalaryAbove { min_salary } => &customer_params.salary >= min_salary,
        Predicate::AgeAbove { min_age } =>  &customer_params.age >= min_age,
        Predicate::InGoodOccupations { occupations } => occupations.contains(&customer_params.occupation),
        Predicate::DEFAULT => panic!("Default predicate"),
    }
}

fn is_date_before(x: &String, y: &String) -> bool {
    let fmt = "%Y-%m-%d %H:%M:%S";
    Local.datetime_from_str(&x, fmt).unwrap().timestamp_millis() < Local.datetime_from_str(&y, fmt).unwrap().timestamp_millis()
}