use phf::phf_map;
use std::{collections::HashMap, error::Error};

use pyo3::prelude::*;

/// Takes a word and a case, and returns it with lithuanian accent marks.
///
/// # Examples
///
/// ```
/// use lithuanian_phonology::get_accentuation;
///
/// assert_eq!(get_accentuation("gera", "Vardininkas"), String::from("gerà"));
/// assert_eq!(get_accentuation("gera", "UNKNOWN"), String::from("gẽra"));
/// assert_eq!(get_accentuation("žodį", "Galininkas"), String::from("žõdį"));
/// ```
pub fn get_accentuation(word: &str, case: &str) -> Result<String, Box<dyn Error>> {
    Python::with_gil(|py| {
        let phonology = PyModule::import(py, "phonology_engine")?;
        let pe = phonology.getattr("PhonologyEngine")?.call0()?;

        let version: Vec<HashMap<String, PyObject>> = pe
            .getattr("process")?
            .call((word,), None)?
            .getattr("__next__")?
            .call0()?
            .get_item(0)?
            .get_item(0)?
            .get_item("stress_options")?
            .get_item("decoded_options")?
            .extract()?;

        for i in version {
            let current_case: &str = i.get("grammatical_case").unwrap().extract(py)?;
            if current_case == case {
                let stress_type: u8 = i.get("stress_type").unwrap().extract(py)?;
                let stressed_letter_index: usize =
                    i.get("stressed_letter_index").unwrap().extract(py)?;
                return Ok(create_stresed_word(
                    word,
                    stress_type,
                    stressed_letter_index,
                ));
            }
        }

        Err("Unable to find correct case".into())
    })
}

fn create_stresed_word(word: &str, stress_type: u8, stressed_letter_index: usize) -> String {
    let mut stressed = String::new();
    for (i, c) in word.chars().enumerate() {
        if i == stressed_letter_index {
            stressed.push_str(make_stressed(c, stress_type));
        } else {
            stressed.push(c);
        }
    }
    stressed
}

static STRESS_TYPE_2: phf::Map<char, &str> = phf_map! {
    'ą' => "ą̃",
    'e' => "ẽ",
    'ė' => "ė̃",
    'ę' => "ę̃",
    'į' => "į̃",
    'l' => "l̃",
    'm' => "m̃",
    'o' => "õ",
    'r' => "r̃",
    'ų' => "ų̃",
    'ū' => "ū̃",
    'y' => "ỹ",
};

static STRESS_TYPE_0: phf::Map<char, &str> = phf_map! {
    'a' => "à",
    'i' => "ì",
    'u' => "ù",
};

static STRESS_TYPE_1: phf::Map<char, &str> = phf_map! {
    'ū' => "ū́",
    'e' => "ę́",
    'ė' => "ė́",
    'į' => "į́",
    'ą' => "ą́",
    'ų' => "ų́",
};

static CASE_NAMES: phf::Map<&str, &str> = phf_map! {
    "nominative" => "Vardininkas",
    "genitive" => "Kilmininkas",
    "dative" => "Naudininkas",
    "accusative" => "Galininkas",
    "instrumental" => "Įnagininkas",
    "locative" => "Vietininkas",
    "vocative" => "Šauksmininkas",
};

/// Utility function that takes an english name of a case, and converts it into Lithuanian.
/// Useful when paired with get_accentuation()
///
/// # Examples
///
/// ```
/// use lithuanian_phonology::get_case_name;
///
/// assert_eq!(get_case_name("Nominative"), "Vardininkas");
/// assert_eq!(get_case_name("INSTRUMENTAL"), "Įnagininkas");
/// ```
pub fn get_case_name(case: &str) -> &str {
    match CASE_NAMES.get(&case.to_lowercase()) {
        Some(val) => val,
        None => "UNKNOWN",
    }
}

fn make_stressed<'a>(c: char, stress_type: u8) -> &'a str {
    let map = match stress_type {
        0 => &STRESS_TYPE_0,
        1 => &STRESS_TYPE_1,
        2 => &STRESS_TYPE_2,
        _ => unreachable!(),
    };
    map.get(&c).unwrap()
}
