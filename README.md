# Secrets CLI

It's a simple tool that let's You easly create, manage and use secrets in Your terminal.

## Install

```
cargo install secrets-cli
```

Please make sure that `~/.cargo/bin` is in your PATH.

## Best way to learn is by example!

First create a `~/templates/var` file with the following content:

```
API_KEY=123456
SECRET=abcdef
```

Then by running the following command:

```
sec copy
```

You will copy the content of the `~/templates/var` file into the clipboard:

```
API_KEY=123 \
SECRET=abc \
```

Then create next file `~/templates/file_name` with the following:

```
API_KEY=456
SECRET=def
```

And by running the following command:

```
sec show file_name
```

This will show the content of the file `~/templates/file_name` in your terminal as:

```
API_KEY=456 \
SECRET=def \
```

Special command for those using `fish` shell:

```
sec fish file_name
```

This will show the content of the file `~/templates/file_name` in your terminal as:

```
set -Ux API_KEY 456; \
set -Ux SECRET def; \
```

## Usage

### Set

Set up a `secrets` folder and a `clipboard` command. Default value is `~/secrets` and `xclip` respectively.

```
sec set [path_to_your_secrets_folder] [clipboard_command]
```

### Copy

```
sec copy [file_name]
```

- `file_name` - selected secrets to copy. If not provided, the `var` file will be used.

### Show

```
sec show [file_name]
```

- `file_name` - selected secrets to show. If not provided, the `var` file will be used.

### Fish

```
sec fish [file_name]
```

- `file_name` - selected secrets to show. If not provided, the `var` file will be used.

### Config

```
sec config
```

Print the current configuration.
