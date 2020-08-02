# A very simple Secret Contract

This is a very simple Secret Contract which can be deployed on the [Secret Network](https://github.com/enigmampc/SecretNetwork), created as part of the [Secret Games incentivised testnet phase 1](https://blog.scrt.network/secret-games-update-testnet-phase-1/). It was built using the [Secret Contracts Starter Pack](https://github.com/enigmampc/secret-template/). For the basics on how to build, test, deploy and interact with secret contracts, please start there. 

The example calls below assume the default situation on the local containerised development environment:
- a key named `a`
- keyring-backend `test`

# Transactional functions

## add_squared: Add the square of a number to a running total

Add the square of a the passed number to a running total.

Example call:

```
secretcli tx compute execute $CONTRACT '{"add_squared": {"amount": 5}}' --from a --keyring-backend test
```

## reset: Reset the running total

Reset the running total to an arbitrary number.

Example call:

```
secretcli tx compute execute $CONTRACT '{"reset": {"total": 0}}' --from a --keyring-backend test
```

# Queryable fields

## get_total

Get the current running total.

```
secretcli query compute contract-state smart $CONTRACT '{"get_total": {}}'
```
