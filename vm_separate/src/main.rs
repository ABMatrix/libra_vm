use types::transaction::SignedTransaction;
use language_e2e_tests::{
    account::AccountData,
    common_transactions::peer_to_peer_txn,
    executor::FakeExecutor,
};

fn main() {
    let mut executor = FakeExecutor::from_genesis_file();
    let sender = AccountData::new(1_000_000, 10);
    let receiver = AccountData::new(100_000, 10);
    executor.add_account_data(&sender);

    executor.add_account_data(&receiver);

    let transfer_amount = 1_000;
    let txn = peer_to_peer_txn(sender.account(), receiver.account(), 10, transfer_amount);

    //println!("{:?}",txn);

    // execute transaction
    let txns: Vec<SignedTransaction> = vec![txn];
    let output = executor.execute_block(txns);

    println!("{:?}",output);
}
