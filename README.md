# Lacewing

## Table of Contents
+ [Getting Started](#getting_started)
+ [Usage](#usage)
+ [Deployment](#deployment)
+ [Contributing](../CONTRIBUTING.md)
+ [Acknowledgements](#acknowledgements)


## Getting Started <a name = "getting_started"></a>
These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

```shell
# Install Cargo tooling
cargo install cornucopia cargo-watch

# Install NPM dependencies
export DBMATE_VERSION=2.7.0
npm install -g dbmate@$DBMATE_VERSION

# Install Mold - Fast Rust Linker
export MOLD_VERSION=2.3.2
curl -OL https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-x86_64-linux.tar.gz
tar -xf mold-$MOLD_VERSION-x86_64-linux.tar.gz
sudo mv ./mold-$MOLD_VERSION-x86_64-linux/bin/mold /usr/bin/
sudo mv ./mold-$MOLD_VERSION-x86_64-linux/lib/mold/mold-wrapper.so /usr/bin/
rm mold-$MOLD_VERSION-x86_64-linux.tar.gz
rm -rf ./mold-$MOLD_VERSION-x86_64-linux
sudo chmod +x /usr/bin/mold

```


### Installing

A step by step series of examples that tell you how to get a development env running.

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo.

## Usage <a name = "usage"></a>

Add notes about how to use the system.

## Deployment <a name = "deployment"></a>

Add notes about how to use the system.

## Acknowledgements <a name = "acknowledgements"></a>
- [Rust on Nails](https://rust-on-nails.com/) - Inspiration and basic framework