# albert

A Substrate node template for experimental and development purposes only.

## What is different

* Full node only, no light client
* Uses a hybrid between `--dev` and `--chain=local` by default
    * `Alice` and `Bob` as authorities
    * `Alice`, `Bob` and `Charlie` as endowed accounts
* No Prometheus and no telemetry
* No pallet template included. Best used following the [pallet as a crate](https://substrate.dev/docs/en/tutorials/create-a-pallet/) approach

Note that `albert` follows [Substrate](https://github.com/paritytech/substrate) `master`. We do check in `Cargo.lock` for a somewhat controlled update process of [Substrate](https://github.com/paritytech/substrate), however.
