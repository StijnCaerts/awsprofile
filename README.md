# `awsprofile`

A TUI application in Rust that helps you select and source AWS profiles with fuzzy matching.

## Features

- 🔍 **Fuzzy search** AWS profiles using skim (Rust implementation of fzf)
- ⚡ **Fast and interactive** profile selection
- 🔐 Automatically exports AWS credentials as environment variables
- 🎯 Easy integration with your shell

## Installation

## Install binary
```bash
cargo binstall awsprofile
```

### Compile and install
```bash
cargo install awsprofile --locked
```

## Usage

### Basic usage

Run the application and select a profile:

```bash
awsprofile
```

### Source the output (recommended)

To actually set the environment variables in your current shell:

```bash
eval $(awsprofile)
```

### Create a shell alias

Add this to your `~/.zshrc` or `~/.bashrc`:

```bash
alias awsp='eval $(awsprofile)'
```

Then simply run:

```bash
awsp
```

## How it works

1. The application reads AWS profiles from `~/.aws/credentials` and `~/.aws/config`
2. Displays an interactive fuzzy finder to select a profile
3. Outputs shell export commands for the selected profile's credentials
4. When eval'd, sets the variables to your environment

## AWS Credentials File Format

Your `~/.aws/credentials` file should look like this:

```ini
[default]
aws_access_key_id = YOUR_ACCESS_KEY_ID
aws_secret_access_key = YOUR_SECRET_ACCESS_KEY

[dev]
aws_access_key_id = DEV_ACCESS_KEY_ID
aws_secret_access_key = DEV_SECRET_ACCESS_KEY

[prod]
aws_access_key_id = PROD_ACCESS_KEY_ID
aws_secret_access_key = PROD_SECRET_ACCESS_KEY
```

And your `~/.aws/config` file should look like this:
```ini
[profile dev]
region = us-west-2

[profile prod]
region = us-east-1
```