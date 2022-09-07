# Cofund Contract

Cofund contract is a contract factory that will create contracts for every resource that a user wants to fund using Cofund. 

There can be multiple types of fundings and each is implemented in a subfolder as a smart contract in this repo. You can find a list of types of funding in the `Cargo.toml` in the `workspace` section. 

## hints 
### deploy
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```

### build
`./build.sh`
### test:unit
`cargo test`
### test:integration
`./build.sh && cd integration-tests && cargo run --example integration-tests \"../contract/target/wasm32-unknown-unknown/release/hello_near.wasm\"`

## interact with the contract using the command line

## read-only methods

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the greeting
near view <dev-account> <view-method>
```

## change methods

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction.

```bash
# Use near-cli to set a new greeting
near call <dev-account> <method>'{"parameter":"value"}' --accountId <dev-account>
```

**Tip:** If you would like to call `set_greeting` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.
