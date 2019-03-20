# Account Library For CryptoNote Based Crypto Currencies


[![](https://travis-ci.com/cryptonote-rust/account.svg?branch=master)](https://travis-ci.com/cryptonote-rust/account)
[![](https://img.shields.io/crates/v/cryptonote-account.svg)](https://crates.io/crates/cryptonote-account)
[![codecov](https://codecov.io/gh/cryptonote-rust/account/branch/master/graph/badge.svg)](https://codecov.io/gh/cryptonote-rust/account)



# Usage

```
    let prefix = 0x3d;
    let acc: Account = Account::new(prefix);

    // Get Keys
    let spendKey = acc.address.spend;
    let viewKey = acc.address.view;

    // Get Address
    let address = acc.get_address();
```