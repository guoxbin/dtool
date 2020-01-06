# dtool

[![Build Status](https://travis-ci.org/guoxbin/dtool.svg?branch=master)](https://travis-ci.org/guoxbin/dtool)
[![Crates.io](https://img.shields.io/crates/v/dtool)](https://crates.io/crates/dtool)

`dtool` is a command line tool collection to assist development

## Table of Contents

- [Description](#description)
- [Usage](#usage)
  - [Tips](#tips)
- [Installation](#installation)

## Description

Now `dtool` supports: 

- Hex / UTF-8 string conversion
- Hex / binary conversion
- Timestamp / date conversion
- Number 2/8/10/16 base conversion
- Hex / base58 conversion
- Hex / base58check conversion
- Hex / base64 conversion
- URL encode / decode
- Number codec
- Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD-160, CRC32)
- UTF-8 string / unicode conversion
- HTML entity encode / decode
- Regex match
- Pbkdf2
- Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab)

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
### Via cargo
```
$ cargo install dtool
```
