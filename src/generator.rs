use crate::types::{PostData, PostFields};

use rand::seq::SliceRandom;
use rand::Rng;

use std::sync::{Arc, Mutex};

pub fn generate_from_fields(
    fields: PostFields,
    domain: String,
    password_list: Arc<Mutex<Vec<String>>>,
    use_password_list: bool,
) -> PostData {
    let mut data = PostData::default();
    let fname = fakeit::name::first();
    if fields.fname.is_some() {
        data.fname = Some(fname.clone());
    }
    let lname = fakeit::name::last();
    if fields.lname.is_some() {
        data.lname = Some(lname.clone());
    }
    let ssn = fakeit::person::ssn();
    if fields.lname.is_some() {
        data.ssn = Some(ssn.clone());
    }
    if fields.email.is_some() {
        data.email = Some(generate_email(fname, lname, domain));
    }
    if fields.password.is_some() {
        if use_password_list {
            data.password = Some(
                password_list
                    .lock()
                    .unwrap()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
            );
        } else {
            data.password = Some(fakeit::password::generate(
                true,
                true,
                true,
                rand::thread_rng().gen_range(5..16),
            ));
        }
    }
    if fields.phone.is_some() {
        data.phone = Some(fakeit::contact::phone());
    }
    if fields.ccn.is_some() {
        let mut ccn = fakeit::payment::credit_card_number();
        while !luhn::valid(ccn.as_str()) {
            ccn = fakeit::payment::credit_card_number();
        }
        data.ccn = Some(ccn);
        data.exp = Some(fakeit::payment::credit_card_exp());
        data.cvv = Some(fakeit::payment::credit_card_cvv());
    }
    return data;
}

fn generate_email(fname: String, lname: String, domain: String) -> String {
    let mut dom = domain.clone();
    if domain.len() == 0 {
        dom = match rand::thread_rng().gen_range(0..11) {
            0 => fakeit::internet::domain_name(),
            1..=2 => "gmail.com".to_string(),
            3..=4 => "yahoo.com".to_string(),
            5..=6 => "aol.com".to_string(),
            7..=8 => "hotmail.com".to_string(),
            9..=10 => "netzero.net".to_string(),
            11 => "mail.com".to_string(),
            _ => fakeit::internet::domain_name(),
        };
    }
    let rand_digit: u8 = rand::thread_rng().gen_range(0..9);
    return match rand::thread_rng().gen_range(0..5) {
        0 => format!("{}{}@{}", fname, lname, dom).to_lowercase(),
        1 => format!("{}{}@{}", fname.get(0..1).unwrap(), lname, dom).to_lowercase(),
        2 => format!("{}{}@{}", fname, lname.get(0..1).unwrap(), dom).to_lowercase(),
        3 => format!("{}{}{}@{}", fname, lname, rand_digit, dom).to_lowercase(),
        4 => format!(
            "{}{}{}@{}",
            fname.get(0..1).unwrap(),
            lname,
            rand_digit,
            dom
        )
        .to_lowercase(),
        5 => format!(
            "{}{}{}@{}",
            fname,
            lname.get(0..1).unwrap(),
            rand_digit,
            dom
        )
        .to_lowercase(),
        _ => format!("{}{}@{}", fname, lname, dom).to_lowercase(),
    };
}
