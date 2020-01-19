# dtool

[![Build Status](https://travis-ci.org/guoxbin/dtool.svg?branch=master)](https://travis-ci.org/guoxbin/dtool)
[![Crates.io](https://img.shields.io/crates/v/dtool)](https://crates.io/crates/dtool)

`dtool` is a command-line tool collection to assist development

## Table of Contents

- [Description](#description)
- [Usage](#usage)
  - [Tips](#tips)
- [Installation](#installation)

## Description

Now `dtool` supports: 

- [Hex / UTF-8 string / binary conversion](./docs/Usage.md#hex--utf-8-string--binary-conversion)
- [Timestamp / date conversion](./docs/Usage.md#timestamp--date-conversion)
- [Number 10/2/8/16 base conversion](./docs/Usage.md#number-102816-base-conversion)
- [Hex / base58 conversion](./docs/Usage.md#hex--base58-conversion)
- [Hex / base64 conversion](./docs/Usage.md#hex--base64-conversion)
- [URL encode / decode](./docs/Usage.md#url-encode--decode)
- [Number codec](./docs/Usage.md#number-codec)
- [Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD, CRC, Blake2b, SM3)](./docs/Usage.md#hash-md5-sha-1-sha-2-sha-3-ripemd-crc-blake2b-sm3)
- [UTF-8 string / unicode conversion](./docs/Usage.md#utf-8-string--unicode-conversion)
- [HTML entity encode / decode](./docs/Usage.md#html-entity-encode--decode)
- [Regex match](./docs/Usage.md#regex-match)
- [Pbkdf2](./docs/Usage.md#pbkdf2)
- [Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab)](./docs/Usage.md#case-conversion-upper-lower-title-camel-pascal-snake-shouty-snake-kebab)
- [AES encrypt / decrypt](./docs/Usage.md#aes-encrypt--decrypt)
- [ECDSA (Secp256k1, NIST P-256, NIST P384)](./docs/Usage.md#ecdsa-secp256k1-nist-p-256-nist-p384)

## Usage

`dtool` does different works by different sub commands:

|Sub command|           Desc            |                                                                                      Example                                                                                      |        Remark        |Since |
|-----------|---------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------|------|
|    h2s    |Convert hex to UTF-8 string|                                                                          $ dtool h2s 0x61626364<br>abcd                                                                           |                      |v0.1.0|
|    s2h    |Convert UTF-8 string to hex|                                                                          $ dtool s2h abcd<br>0x61626364                                                                           |                      |v0.1.0|
|    ...    |...|                                                                          ...                                                                           |                      |...|

[View full usage document](./docs/Usage.md)

* Besides the sub command `help`, `dtool` provides a new sub command `usage` to show examples:

```bash
$ dtool usage
Usage

----------------------------------------------------------------------------------
 Sub command  Desc                         Example                 Remark  Since 
==================================================================================
 h2s          Convert hex to UTF-8 string  $ dtool h2s 0x61626364          v0.1.0 
                                           abcd                             
----------------------------------------------------------------------------------
...
```

* You can search usage with a keyword:
```bash
$ dtool usage -s md5
------------------------------------------------------------------------------
 Sub command  Desc         Example                             Remark  Since 
==============================================================================
 hash         Hex to hash  $ dtool hash -a md5 0x616263        MD5     v0.2.0 
                           0x900150983cd24fb0d6963f7d28e17f72           
------------------------------------------------------------------------------

```

## Tips

### Hex / UTF-8 string / binary conversion

### pipe 
convert a string to base64
```
$ echo -n abc | dtool s2h | dtool h2b64
YWJj
```

convert a encoded timestamp to date
```
$ echo -n 2c28e75d | dtool nd -tu32 | dtool ts2d
2019-12-04 11:29:48
```

convert a jpeg to base64
```
$ cat pic.jpg | dtool b2h | dtool h2b64
/9j/4AAQSkZJR...
```

calculate file md5
```
$ cat pic.jpg | dtool b2h | dtool hash -a md5
0x1884b72e23b0c93320bac6b050478ff4
```

## Installation
### Via brew 
```bash
$ brew install guoxbin/guoxbin/dtool
```
Recommend! Brew will install `dtool bash completion` along with `dtool`

### Via cargo
```bash
$ cargo install dtool
```

