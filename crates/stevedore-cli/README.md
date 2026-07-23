# stevedore-cli

The command-line interface for
[`stevedore`](https://github.com/ojhermann-org/stevedore) — move secrets between
password managers and vaults. Installs the `stevedore` binary.

```console
$ stevedore stores
sources: dashlane
sinks:   proton
routes:  dashlane -> proton (in progress)

$ stevedore migrate --from dashlane --to proton --input export.csv
# dry-run by default; add --apply to write. (route not implemented yet)
```

> Early scaffold — the `migrate` command reports honestly that the route isn't
> implemented yet while the Dashlane → Proton path is built. Dry-run is the safe
> default; `--apply` is required to write to a sink.

Licensed under either of Apache-2.0 or MIT at your option.
