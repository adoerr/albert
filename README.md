# albert

A Substrate node template for experimental and development purposes only.

## What is different

* Full node only, no light client
* Uses a hybrid between `--dev` and `--chain=local` by default
    * `Alice` and `Bob` as authorities
    * `Alice`, `Bob` and `Charlie` as endowed accounts
* No Prometheus and no telemetry
* No pallet template included. Best used following the [pallet as a crate](https://substrate.dev/docs/en/tutorials/create-a-pallet/) approach

Note that `albert` follows [Substrate](https://github.com/paritytech/substrate) `master`. __However__, instead of Substrate upstream, `albert` follows [this](https://github.com/adoerr/substrate) fork. Reason is that this allows us to follow
`master` closely while sill having some sort of well-known version of Substrate (current head of the fork). 