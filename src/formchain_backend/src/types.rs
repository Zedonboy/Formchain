use std::{borrow::Cow, collections::HashMap, fmt};

use candid::CandidType;
use hex::encode;
use ic_stable_structures::{memory_manager::VirtualMemory, storable::Bound, BTreeMap, DefaultMemoryImpl, Storable};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

type Memory = VirtualMemory<DefaultMemoryImpl>;

type FormID = String;

#[derive(Serialize, Deserialize, CandidType)]

pub struct Form {
    name : String,
}

#[derive(Clone)]
pub enum FormDataType {
    Text(String),
    Num(i32),
    Bool(bool),
    Select(String),
    MultiSelect(Vec<String>),
    Rating(u32),
    Scale(String)
}

pub struct FormQuestion {
    id: u32,
    question: String,
    data_type: FormDataType,
    default: Option<FormDataType>
}

#[derive(Clone)]
pub struct FormAnswer {
    id: u32,
    answer: FormDataType
}

#[derive(Clone)]
pub struct FormEntry {
    answers: Vec<FormAnswer>
}



#[derive(Serialize, Deserialize)]
pub struct Ledger {
    forms : HashMap<FormID, Form>,
    entries : HashMap<FormID, Vec<Map<String, Value>>>
}

#[derive(CandidType)]
pub enum FormChainError {
    InternalSystemError(String),
    KeyNotExist
}

impl fmt::Display for FormChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let representation = match self {
            FormChainError::InternalSystemError(mssg) => format!("Internal System Error:{}", mssg),
            FormChainError::KeyNotExist => "Key does not exist".to_string(),
        };
        write!(f, "{}", representation)
    }
}

impl Ledger {

    pub fn new() -> Self {
        Ledger {
            forms : HashMap::new(),
            entries : HashMap::new()
        }
    }

    pub fn add_entry(&mut self, form_id : FormID, map : Map<String, Value>) -> Result<bool, FormChainError> {
        if !self.forms.contains_key(&form_id) {
            return Err(FormChainError::KeyNotExist);
        }

        let entry_opt = self.entries.entry(form_id).or_insert(vec![]);
        entry_opt.push(map);
        Ok(true)
        
    }
    
    pub fn create_form(&mut self, form : Form, rnd_bytes : Vec<u8>) -> Result<String, FormChainError> {
        
        let form_id = encode(rnd_bytes);
        if self.forms.contains_key(&form_id) {
            return Err(FormChainError::InternalSystemError("System generated a key that exists".to_string()));
        }
        self.forms.insert(form_id.clone(), form);
        Ok(form_id)
    }

    

    // pub async fn submit_form(mut self, formId : FormID, entry : FormEntry) -> Result<(), FormChainError> {
    //     if !self.forms.contains_key(&formId) {
    //         return Err(FormChainError::KeyNotExist);
    //     };

    //     let vec_opt = self.entries.get_mut(&formId);

    //     if vec_opt.is_none() {
    //         let new_entries_vec = vec![entry];
    //         self.entries.insert(formId, new_entries_vec);
    //     } else {
    //         let entries_vec = vec_opt.unwrap();
    //         entries_vec.push(entry);
    //     }

    //     Ok(())
    // }

    pub async fn get_entries_count(self, form_id : FormID) -> Result<u32, FormChainError> {
        let entries = self.entries.get(&form_id);

        if entries.is_some() {
            let vec = entries.unwrap();
            return Ok(vec.len() as u32);
        } else {
            return Ok(0u32);
        }
    }

    pub async fn get_entries(self, form_id : FormID) -> Result<Vec<Map<String, Value>>, FormChainError> {
        let entries_opt = self.entries.get(&form_id);
        if entries_opt.is_none() {
            return Ok(vec![]);
        } else {
            let entries = entries_opt.cloned().unwrap();
            return Ok(entries);
        }
    }
}


impl Storable for Ledger {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::into_writer(self, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ciborium::from_reader(bytes.as_ref()).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = Bound::Unbounded;
}