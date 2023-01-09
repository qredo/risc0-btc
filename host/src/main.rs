// TODO: Update the name of the method loaded by the prover. E.g., if the method is `multiply`, replace `METHOD_NAME_ID` with `MULTIPLY_ID` and replace `METHOD_NAME_PATH` with `MULTIPLY_PATH`
use methods::{MULTIPLY_ID, MULTIPLY_PATH};
// use risc0_zkvm::Prover;
// use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{
    Prover, ProverOpts,
};

/*
    let mut prover = Prover::new(MULTI_TEST_ELF, MULTI_TEST_ID).unwrap();
    prover.add_input_u32_slice(&to_vec(&MultiTestSpec::ShaDigest { data: msg.into() }).unwrap());
    let receipt = prover.run().unwrap();
    from_slice::<Digest>(&receipt.journal).unwrap().into_orig()
*/

fn main() {
    // Make the prover.
    let method_code = std::fs::read(MULTIPLY_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let opts = ProverOpts::default().with_skip_seal(true);
    let mut prover = Prover::new_with_opts(&method_code, MULTIPLY_ID, opts).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // prover.add_input_u32_slice(&to_vec(&MultiTestSpec::ShaDigest { data: msg.into() }).unwrap());
    prover.   add_input_u32_slice(&[0,1,2]);
    // TODO: Implement communication with the guest here
    println!("Foo");

    // Run prover & generate receipt
    let _receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    // receipt.verify(MULTIPLY_ID).expect(
    //     "Code you have proven should successfully verify; did you specify the correct method ID?",
    // );

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}