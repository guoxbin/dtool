# dtool

[![Build Status](https://img.shields.io/github/actions/workflow/status/guoxbin/dtool/rust.yml?branch=master&style=flat-square)](https://github.com/guoxbin/dtool/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/dtool?style=flat-square)](https://crates.io/crates/dtool)
[![Crates.io](https://img.shields.io/crates/d/dtool?style=flat-square)](https://crates.io/crates/dtool)
[![License](https://img.shields.io/badge/license-GNU%203.0-blue?style=flat-square)](LICENSE-GNU)
[![Contributors](https://img.shields.io/github/contributors/guoxbin/dtool?style=flat-square)](https://github.com/guoxbin/dtool/graphs/contributors)


`dtool` is a command-line tool collection to assist development

## Table of Contents

- [Description](#description)
- [Usage](#usage)
  - [Tips](#tips)
- [Installation](#installation)

## Description

Now `dtool` supports: 

- [Hex / UTF-8 string / binary / byte array conversion](./docs/Usage.md#hex--utf-8-string--binary--byte-array-conversion)
- [Timestamp / date conversion](./docs/Usage.md#timestamp--date-conversion)
- [Number 10/2/8/16 base conversion](./docs/Usage.md#number-102816-base-conversion)
- [Hex / base58 conversion](./docs/Usage.md#hex--base58-conversion)
- [Hex / base64 conversion](./docs/Usage.md#hex--base64-conversion)
- [URL encode / decode](./docs/Usage.md#url-encode--decode)
- [Number codec](./docs/Usage.md#number-codec)
- [Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD, CRC, Blake2b, SM3, Twox)](./docs/Usage.md#hash-md5-sha-1-sha-2-sha-3-ripemd-crc-blake2b-sm3-twox)
- [UTF-8 string / unicode conversion](./docs/Usage.md#utf-8-string--unicode-conversion)
- [HTML entity encode / decode](./docs/Usage.md#html-entity-encode--decode)
- [Regex match](./docs/Usage.md#regex-match)
- [Pbkdf2](./docs/Usage.md#pbkdf2)
- [Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab, sarcasm)](./docs/Usage.md#case-conversion-upper-lower-title-camel-pascal-snake-shouty-snake-kebab-sarcasm)
- [AES encrypt / decrypt](./docs/Usage.md#aes-encrypt--decrypt)
- [ECDSA (Secp256k1, NIST P-256, NIST P-384, SM2)](./docs/Usage.md#ecdsa-secp256k1-nist-p-256-nist-p-384-sm2)
- [SM4 encrypt / decrypt](./docs/Usage.md#sm4-encrypt--decrypt)
- [EdDSA (Ed25519)](./docs/Usage.md#eddsa-ed25519)
- [sr25519 signature](./docs/Usage.md#sr25519-signature)

## Usage

`dtool` does different works by different sub commands:

|Sub command|                Desc                 |           Example            |
|-----------|-------------------------------------|------------------------------|
|    h2s    |Convert hex to UTF-8 string<br>v0.1.0|$ dtool h2s 0x61626364<br>abcd|
|    s2h    |Convert UTF-8 string to hex<br>v0.1.0|$ dtool s2h abcd<br>0x61626364|
|    h2b    |   Convert hex to binary<br>v0.1.0   |$ dtool h2b 0x61626364<br>abcd|
|    b2h    |   Convert binary to hex<br>v0.1.0   |$ dtool b2h abcd<br>0x61626364|
|    ...|

[View full usage document](./docs/Usage.md)

* Besides the sub command `help`, `dtool` provides a sub command `usage` to show examples:

```bash
$ dtool usage
Usage
----------------------------------------------------------
 h2s  Convert hex to UTF-8 string  $ dtool h2s 0x61626364 
      v0.1.0                       abcd 
----------------------------------------------------------
...
```

* You can search usage with a keyword:
```bash
$ dtool usage -s md5
Usage
-------------------------------------------------------
 hash  Hex to hash  $ dtool hash -a md5 0x616263 
       MD5          0x900150983cd24fb0d6963f7d28e17f72 
       v0.2.0        
-------------------------------------------------------
```

## Tips
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
### Homebrew 
```bash
$ brew install guoxbin/guoxbin/dtool
```
Recommend! Homebrew will install shell completion for bash, fish, and zsh along with `dtool`

### Arch Linux

There is [an AUR package for dtool](https://aur.archlinux.org/packages/dtool/) that includes shell completion for bash, fish, and zsh.

```bash
git clone https://aur.archlinux.org/dtool.git
cd dtool
makepkg -si
```

### Cargo
```bash
$ cargo install dtool
```

