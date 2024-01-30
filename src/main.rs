use snarkvm::ledger::store::ConsensusStore;
use snarkvm::ledger::store::helpers::memory::ConsensusMemory;
use snarkvm::prelude::{Address, Literal, Plaintext, PrivateKey, U64, Uniform, Value, VM};

pub type CurrentNetwork = snarkvm::prelude::Testnet3;

fn main() {
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a VM.
    let vm = VM::from(ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None).unwrap()).unwrap();

    // Initialize a private key.
    let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();

    // Initialize a random address.
    let pk = PrivateKey::<CurrentNetwork>::new(rng).unwrap();
    let address = Address::try_from(pk).unwrap();

    // Time the execution of `transfer_public`.
    let time = std::time::Instant::now();

    // Execute `transfer_public`.
    let _ = vm.execute(
        &private_key,
        ("credits.aleo", "transfer_public"),
        [
            Value::Plaintext(Plaintext::from(Literal::Address(address))),
            Value::Plaintext(Plaintext::from(Literal::U64(U64::new(100u64))))
        ].iter(),
        None,
        0,
        None,
        rng,
    ).unwrap();

    // Print the execution time.
    println!("Execution time: {:?}", time.elapsed());
}