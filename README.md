## About 

This repository contains benchmarking tools to compare different RPC endpoints against a list of Starknet JSON RPC methods. Powered by the [Criterion](https://github.com/bheisler/criterion.rs) benchmarking library.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- In order to generate plots, you should have [gnuplot](http://www.gnuplot.info/) installed

## Usage

### Configuration file

Edit the configuration file at `/config/config.json` to add the RPC endpoints and RPC methods of your choice. The format is as follows:

```json
{
    "targets" : [
        {"name": "RPC_NAME", "url": "RPC_URL"},
        ...
    ],
    "methods" : ["METHOD_NAME", ... ],
    "params" : {
        "block": "NUMBER",
        "class_hash": "CLASS_HASH",
        "tx_hash": "TX_HASH"
    }
}
```
To bench different endpoints, add the pairs "name", "url" to the "targets" array (the "name" field is a custom name of your choice that will appear in the reports).  The methods implemented by this library (i.e. the available options for METHOD_NAME above) are:

- starknet_blockNumber
- starknet_getBlockWithTxHashes
- starknet_getBlockWithTxs
- starknet_getStateUpdate
- starknet_getTransactionByHash
- starknet_getTransactionByBlockIdAndIndex
- starknet_getTransactionReceipt
- starknet_getClass
- starknet_getClassHashAt
- starknet_call 

Contrary to the other methods, the input parameters for `starknet_call` are currently hardcoded by some constants.


### Run

From the project directory, run 

```bash
cargo bench
```

### Reports

The output report (including statistics and graphics) will be available at `/target/criterion/report/index.html`.


