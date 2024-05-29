use std::{cell::RefCell, collections::HashMap, sync::Arc};

use candid::Principal;
use ic_cdk::{self, api::{data_certificate, management_canister::main::{create_canister, install_chunked_code, install_code, CanisterSettings, CreateCanisterArgument, InstallCodeArgument}}, call, caller, id, query, update};
use ic_ledger_types::{account_balance, transfer, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID};
use serde_bytes::ByteBuf;
use types::{NotifyTopUpArg, NotifyTopUpResult, Rcbytes, RegistryError};
mod types;

pub const TRANSACTION_FEE: Tokens = Tokens::from_e8s(1000);
pub const MEMO_MINT_CYCLES: u64 = 0x544e494d;
const NOTIFY_TOP_UP_METHOD: &str = "notify_top_up";


thread_local! {
    // key is principal id and value is storage canister 
    static PREMIUM_USERS : RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static USER_STORAGE : Rcbytes = Rcbytes(Arc::new(ByteBuf::from(include_bytes!("formchain_backend.wasm"))))
}
#[update(guard = "not_anonymous")]
async fn create_user() -> Result<String, RegistryError> {
   let user_exists = PREMIUM_USERS.with_borrow(|store| {
        store.contains_key(&caller().to_text())
    });

    if user_exists {
        return Err(RegistryError::UserIdExists);
    }

    // check if user has deposited ICP.
    let account = AccountIdentifier::new(&id(), &Subaccount::from(caller()));

    let balance_args = AccountBalanceArgs{ account };

    let user_balance_tokens = account_balance(MAINNET_LEDGER_CANISTER_ID, balance_args).await.unwrap();

    let amount = user_balance_tokens.e8s();

    if amount < 50000 {
        return Err(RegistryError::AmountBelowMin);
    }

    let cycles_acc = AccountIdentifier::new(&MAINNET_CYCLES_MINTING_CANISTER_ID, &Subaccount::from(id()));

    let transger_args = TransferArgs { memo: Memo(MEMO_MINT_CYCLES), amount: user_balance_tokens, fee: TRANSACTION_FEE, from_subaccount: Some(Subaccount::from(caller())), to: cycles_acc, created_at_time: None };

    let block_height = transfer(MAINNET_LEDGER_CANISTER_ID, transger_args).await.unwrap().unwrap();

    let notify_args = NotifyTopUpArg { block_index: block_height, canister_id: id() };

    let (rslt,) : (NotifyTopUpResult, ) = call(MAINNET_CYCLES_MINTING_CANISTER_ID, NOTIFY_TOP_UP_METHOD, (notify_args,)).await.unwrap();

    let deposited_cycles = rslt.unwrap();

    let settings = CanisterSettings { controllers: Some(vec![caller(), id()]), compute_allocation: None, memory_allocation: None, freezing_threshold: None, reserved_cycles_limit: None };

    let create_arg = CreateCanisterArgument { settings: Some(settings) };

    let (id_record, ) = create_canister(create_arg, deposited_cycles).await.unwrap();

    let Rcbytes(wasm_bytes) = USER_STORAGE.with(|f| f.clone());

    let install_arg = InstallCodeArgument { mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install, canister_id: id_record.canister_id, wasm_module: wasm_bytes.to_vec(), arg: vec![] };

    install_code(install_arg).await.unwrap();
    
    PREMIUM_USERS.with_borrow_mut(|store| {
        store.insert(caller().to_text(), id_record.canister_id.to_text())
    });

    Ok(id_record.canister_id.to_text())
}

#[query(guard = "not_anonymous")]
async fn deposit_address() -> Result<String, RegistryError> {
    let account = AccountIdentifier::new(&id(), &Subaccount::from(caller()));
    Ok(account.to_hex())
}

#[query(guard = "not_anonymous")]
async fn is_premium_user() -> Result<bool, ()> {
    let answer = PREMIUM_USERS.with_borrow(|store| {
        store.contains_key(&caller().to_text())
    });

    Ok(answer)
}

fn not_anonymous() -> Result<(), String> {
    if caller()  == Principal::anonymous() {
        return Err("You are anonymous".to_string());
    }

    Ok(())
}
