# Account Library For CryptoNote Based Crypto Currencies


[![](https://travis-ci.com/cryptonote/account.svg?branch=master)](https://travis-ci.com/cryptonote/account)
[![](https://img.shields.io/crates/v/cryptonote-account.svg)](https://crates.io/crates/cryptonote-account)
[![codecov](https://codecov.io/gh/cryptonote/account/branch/master/graph/badge.svg)](https://codecov.io/gh/cryptonote/account)



# Usage

```
    let prefix = 0x3d;
    let acc: Account = Account::new(prefix);

    // Get Keys
    let spendKey = acc.keys.spend;
    let viewKey = acc.keys.view;

    // Get Address
    let address = acc.getAddress();
```