# Ascella Uploader

[![Crowdin](https://badges.crowdin.net/ascella-uploader/localized.svg)](https://crowdin.com/project/ascella-uploader) [![GitHub license](https://img.shields.io/github/license/tricked-dev/ascella)](https://github.com/Tricked-dev/ascella/blob/master/LICENSE) ![Website](https://img.shields.io/website?url=https%3A%2F%2Fascella.host)
Ascella is the fastest opensource image uploader.

![Alt](https://repobeats.axiom.co/api/embed/9b97008a7840311ce626d6162e3c64312ec250cd.svg "Repobeats analytics image")

## Features:

- Custom bot to manage almost everything
- Awesome frontend
- Rust written backend!
- Awesome Dashboard with features (it sucks)
- Fully opensource
- Linux & Mac support using our custom desktop app!
- Awesome community at the [discord](https://discord.gg/mY8zTARu4g)

## Contributing

see [contributing](https://dosc.ascella.host/contributing)

---

### Ascella - Backend

Discord: https://discord.gg/mY8zTARu4g

### Stack

- storage - localstorage ( s3 expensive smfh )
- database - postgresql using tokio postgres
- http - actix-web
- bot - twilight

### Selfhosting

There's no support for self hosting see #1

### Development

You will need to create a .env file with the following entries.

```ini
# Postgresql database
DATABASE_URL=
DISCORD_TOKEN=
APPLICATION_ID=
WEBHOOK=
```

#### Faster Development start

```sh
cargo {run,build} --profile dev
```

#### Contributing

Check the issues tab to find issues that need to be resolved

## Formatting

for formatting we use [dprint](https://dprint.dev/install/) for formatting files
`npm i dprint -g`

### Copyright

please contact me via discord (tricked#3777) before using any of the code i
would greatly appreciate this.

### Donating

You can help keep ascella running by supporting me on
[github sponsers](https://github.com/sponsors/Tricked-dev/)
