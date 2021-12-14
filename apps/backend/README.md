# Ascella - Backend

Discord: https://discord.gg/mY8zTARu4g

## Stack

- storage - localstorage ( s3 expensive smfh )
- database - postgresql using tokio postgres
- http - actix-web
- bot - twilight

## Selfhosting

There's no support for self hosting

## Development

You will need to create a .env file with the following entries.

```ini
# Postgresql database
DATABASE_URL=
DISCORD_TOKEN=
APPLICATION_ID=
WEBHOOK=
```

### Faster Development start

```sh
cargo {run,build} --profile dev
```

### Contributing

Check the issues tab to find issues that need to be resolved

### Donating

<img src="https://tricked.pro/crypto/monero-xmr-logo.png" alt="" height="15px"> `89prBkdG58KU15jv5LTbP3MgdJ2ikrcyu1vmdTKTGEVdhKRvbxgRN671jfFn3Uivk4Er1JXsc1xFZFbmFCGzVZNLPQeEwZc`

<img src="https://tricked.pro/crypto/ethereum-eth-logo.png" alt="" height="15px"> `0xc31a1A5dCd1a4704e81fB7c9C3fa858b9A00C7fb`

<img src="https://tricked.pro/crypto/bitcoin-cash-bch-logo.png" alt="" height="15px"> `qz9gyruyyvtwcmevtcnyru8gudenqjqeug096e459m`
