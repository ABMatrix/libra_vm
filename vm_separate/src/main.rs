use vm_genesis::encode_mint_program;
use types::transaction::RawTransaction;
use chrono::Utc;
use vm_runtime::MoveVM;
use config::config::VMConfig;
use types::account_address::AccountAddress;
use types::transaction_helpers::TransactionSigner;
use nextgen_crypto::ed25519::compat;
use types::account_address::ADDRESS_LENGTH;
use state_view::StateView;
use types::access_path::AccessPath;
pub use failure::{
    _core, bail, ensure, err_msg, format_err, AsFail, Backtrace, Causes, Compat, Context, Error,
    Fail, ResultExt, SyncFailure,
};
use vm_runtime::VMExecutor;

//use nextgen_crypto::{ed25519::*, test_utils::KeyPair, traits::SigningKey};
fn main() {
    let sender = gen_address(0);
    let program = vm_genesis::encode_mint_program(&sender,1000);
    let raw_txn = RawTransaction::new(
        sender,
        0,
        program,
        1000,
        0,
        std::time::Duration::from_secs(0),
    );
    let (privkey, pubkey) = compat::generate_keypair(None);
    let sig_txn = raw_txn
        .sign(&privkey, pubkey)
        .expect("Failed to sign raw transaction.")
        .into_inner();

    let mut txns = vec![];

    txns.push(sig_txn);

    let output = MoveVM::execute_block(
        txns,
        &VMConfig::empty_whitelist_FOR_TESTING(),
        &MockStateView,
    );

    println!("{:?}",output);
}

fn gen_address(index: u8) -> AccountAddress {
    AccountAddress::new([index;ADDRESS_LENGTH])
}

pub type Result<T> = ::std::result::Result<T, Error>;

struct MockStateView;

impl StateView for MockStateView {
    fn get(&self, _access_path: &AccessPath) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn multi_get(&self, _access_paths: &[AccessPath]) -> Result<Vec<Option<Vec<u8>>>> {
        unimplemented!();
    }

    fn is_genesis(&self) -> bool {
        false
    }
}