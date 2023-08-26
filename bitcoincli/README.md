# bitcoincli
[![CircleCI](https://circleci.com/gh/chainstack/bitcoincli/tree/master.svg?style=svg)](https://circleci.com/gh/chainstack/bitcoincli/tree/master)
[![PyPI version](https://badge.fury.io/py/bitcoincli.svg)](https://badge.fury.io/py/bitcoincli)

## Description

Heavily based on [python-bitcoinrpc](https://github.com/jgarzik/python-bitcoinrpc) and [Savoir](https://github.com/DXMarkets/Savoir) but replacing the httplib with [requests](http://docs.python-requests.org/en/master/).

## Installation

``` sh
git clone https://github.com/chainstack/bitcoincli
```

## Usage

After cloning this repository, run:

``` sh
python setup.py develop
```

Use the [Bitcoin API documentacion](https://bitcoin.org/en/developer-reference#bitcoin-core-apis) and make calls to the wrapper.

Remember to replace the RPC variables with your Bitcoin node access credentials.

```python
from bitcoincli import Bitcoin

host = "nd-123-456-789.p2pify.com"
port = "80"
username = "user-name"
password = "pass-word-pass-word-pass-word"

bitcoin = Bitcoin(username, password, host, port)

info = bitcoin.getblockchaininfo()
print(info)
```
