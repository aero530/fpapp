
## Development Setup ##

### Clone the repo via git ###

```cmd
git clone https://github.com/aero530/fpapp.git fpapp
```

## Update TypeScript Bindings ##

The accounts rust module uses ts-rs to automatically create TS bindings for use in the UI. Currently these
need to manually generated if the accounts module changes.

```cmd
> cd src-tauri/src/accounts; cargo test; cd ../../../
```

## Dev ##

Start app in dev mode:

Note that vscode terminal is somehow broken in linux and this fails to run there.  Running it from a os terminal window works fine.

```cmd
> cargo tauri dev
```

## Packaging ##

Create a package for macOS, Windows, or Linux using one of the following commands:

```cmd
> cargo tauri build
```

<!-- ```cmd
> cargo build --release
``` -->

## Tests ##

```cmd
> cargo test
```