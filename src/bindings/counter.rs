use alloy::sol;

// Counter contract bindings generated at compile time from ABI
sol! {
    #[sol(rpc, ignore_unlinked)]
    Counter,
    "src/bindings/abis/Counter.json"
}
