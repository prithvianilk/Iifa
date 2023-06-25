use chrono::{Local, TimeZone};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{Value, from_value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Predicate {
    LTNumber{lhs: String, rhs: String},
    GTNumber{lhs: String, rhs: String},
    EQNumber{lhs: String, rhs: String},
    EQString{lhs: String, rhs: String},
    InListOfString{lhs: String, rhs: String},
    Default
}

impl Default for Predicate {
    fn default() -> Self { Predicate::Default }
}

pub fn evaluate(predicate: &Predicate, input_params: &Value, context: &Value) -> bool {
    match predicate {
        Predicate::LTNumber { lhs, rhs } => lt::<f64>(lhs, rhs, input_params, context),
        Predicate::GTNumber { lhs, rhs } => gt::<f64>(lhs, rhs, input_params, context),
        Predicate::EQNumber { lhs, rhs } => eq::<f64>(lhs, rhs, input_params, context),
        Predicate::EQString { lhs, rhs } => eq::<String>(lhs, rhs, input_params, context),
        Predicate::InListOfString { lhs, rhs } => list_contains::<String>(lhs, rhs, input_params, context),
        Predicate::Default => panic!("Default predicate can't be evaluated"),
    }
}

fn lt<T>(lhs: &String, rhs: &String, input_params: &Value, context: &Value) -> bool
where T: std::cmp::PartialOrd, T: DeserializeOwned {
   let lhs_value = &evaluate_path::<T>(lhs, input_params);
   let rhs_value =  &evaluate_path::<T>(rhs, context);
   lhs_value < rhs_value
}

fn gt<T>(lhs: &String, rhs: &String, input_params: &Value, context: &Value) -> bool
where T: std::cmp::PartialOrd, T: DeserializeOwned {
   let lhs_value = &evaluate_path::<T>(lhs, input_params);
   let rhs_value =  &evaluate_path::<T>(rhs, context);
   lhs_value > rhs_value
}

fn eq<T>(lhs: &String, rhs: &String, input_params: &Value, context: &Value) -> bool
where T: std::cmp::PartialOrd, T: DeserializeOwned {
   let lhs_value = &evaluate_path::<T>(lhs, input_params);
   let rhs_value =  &evaluate_path::<T>(rhs, context);
   lhs_value == rhs_value
}

fn list_contains<T>(lhs: &String, rhs: &String, input_params: &Value, context: &Value) -> bool
where T: std::cmp::PartialOrd, T: DeserializeOwned {
   let lhs_value = &evaluate_path::<T>(lhs, input_params);
   let rhs_value =  &evaluate_path::<Vec<T>>(rhs, context);
   rhs_value.iter().find(|x| *x == lhs_value).is_some()
}

fn evaluate_path<T>(path: &String, context: &Value) -> T where T: DeserializeOwned {
    let value = context.pointer(&path).unwrap().clone();
    from_value(value).unwrap()
}

fn is_date_before(x: &String, y: &String) -> bool {
    let fmt = "%Y-%m-%d %H:%M:%S";
    Local.datetime_from_str(&x, fmt).unwrap().timestamp_millis() < Local.datetime_from_str(&y, fmt).unwrap().timestamp_millis()
}