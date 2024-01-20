# ccanvas-quit

Handles quitting (configurable).

```jsonc
// ~/.config/ccanvas/quit.jsonc

{
  "keys": [
    {
      "type": "char",
      "value": "q",
      "modifier": "ctrl"
    }
    // .. more entries
  ]
}
```

## Config values

|type|value|
|--|--|
|backspace|N/A|
|left|N/A|
|right|N/A|
|up|N/A|
|down|N/A|
|home|N/A|
|end|N/A|
|pageup|N/A|
|pagedown|N/A|
|backtab|N/A|
|delete|N/A|
|insert|N/A|
|f|Integer between 1 to 12|
|char|Any single character|
|null|N/A|
|esc|N/A|

### Modifier keys

- ctrl
- alt
- none
- any

## Usage

First make sure that `ccanvas-quit` has been installed. For exmaple using the `cargo install` command.

```sh
cargo install --git https://github.com/ccanvas/ccanvas-quit
```

Add it to your program.

```rs
// make sure that the process is loaded in the *master space*
client.load("quit".to_string(), "ccanvas-quit".to_string(), Vec::new());
```

Note that `ccanvas-quit` will be listening to key presses with priority 5.
