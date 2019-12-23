# dtool

[![Build Status](https://travis-ci.org/guoxbin/dtool.svg?branch=master)](https://travis-ci.org/guoxbin/dtool)
[![Crates.io](https://img.shields.io/crates/v/dtool)](https://crates.io/crates/dtool)

`dtool` is a collection of development tools:

## Table of Contents

- [Usage](#usage)
- [Tips](#tips)
- [Installation](#installation)

## Usage
| Sub command   | Description   | Examples |  Since | 
| ------------- | :------------- | ------------ | --- |
| h2s    | Convert hex to UTF-8 string | $ dtool h2s 0x61626364 <br> abcd  | v0.1.0 |
| s2h    | Convert UTF-8 string to hex | $ dtool h2s abcd <br> 0x61626364 | v0.1.0 |
| h2b    | Convert hex to binary | $ dtool h2b 0x61626364 > pic.jpg | v0.3.0 |
| b2h    | Convert binary to hex | $ cat pic.jpg &#x7C; dtool b2h <br> 0x61626364 | v0.3.0 |
| ts2d   | Convert timestamp to date   | $ dtool ts2d 10000 <br> 1970-01-01 10:46:40 <br> $ dtool ts2d -z0 10000 <br> 1970-01-01 02:46:40 | v0.1.0 |
| d2ts   | Convert date to timestamp   | $ dtool d2ts '1970-01-01 10:46:40' <br> 10000 <br> $ dtool d2ts -z0 '1970-01-01 02:46:40' <br> 10000 | v0.1.0 |
| d2ts   | Number system               | $ dtool ns 256 <br> 256 <br> 0b100000000 <br> 0o400 <br> 0x100 <br> | v0.1.0 |
| h2b58  | Convert hex to base58       | $ dtool h2b58 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a <br> 12dvBhvPEPniQmBmgvj4qpJEodT7P | v0.1.0 |
| h2b58c | Convert hex to base58 check | $ dtool h2b58c 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a <br> 1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF | v0.1.0 |
| b582h  | Convert base58 to hex       | $ dtool b582h 12dvBhvPEPniQmBmgvj4qpJEodT7P <br> 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a | v0.1.0 |
| b58c2h | Convert base58 check to hex | $ dtool b58c2h 1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF <br> 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a | v0.1.0 |
| h2b64  | Convert hex to base64       | $ dtool h2b64 0x616263 <br> YWJj | v0.1.0 |
| b642h  | Convert base64 to hex       | $ dtool b642h YWJj <br> 0x616263 | v0.1.0 |
| ue     | URL encode                  | $ dtool ue a+b <br> a%2Bb | v0.1.0 |
| ud     | URL decode                  | $ dtool ud a%2Bb <br> a+b | v0.1.0 |
| ne     | Number encode               | $ dtool ne -tu8 1 <br> 0x01 <br> $ dtool ne -tu16 1 <br> 0x0100 <br> $ dtool ne -tu32 1 <br> 0x01000000 <br> $ dtool ne -tu64 1 <br> 0x0100000000000000 <br> $ dtool ne -tu128 1 <br> 0x01000000000000000000000000000000 <br> $ dtool ne -tc 6 <br> 0x18 <br> dtool ne -tc 251 <br> 0xed03 | v0.1.0 |
| nd     | Number decode               | $ dtool nd -tu8 0x01 <br> 1 <br> $ dtool nd -tu16 0x0100 <br> 1 <br> $ dtool nd -tu32 0x01000000 <br> 1 <br> $ dtool nd -tu64 0x0100000000000000 <br> 1 <br> $ dtool nd -tu128 0x01000000000000000000000000000000 <br> 1 <br> $ dtool nd -tc 0x18 <br> 6 <br> $ dtool nd -tc 0xed03 <br> 251 | v0.1.0 |
| hash   | Convert hex to MD5          | $ dtool hash -a md5  0x616263 <br> 0x900150983cd24fb0d6963f7d28e17f72 | v0.2.0 |
| hash   | Convert hex to SHA-1        | $ dtool hash -a sha1 0x616263 <br> 0xa9993e364706816aba3e25717850c26c9cd0d89d | v0.2.0 |
| hash   | Convert hex to SHA-2 224    | $ dtool hash -a sha2_224 0x616263 <br> 0x23097d223405d8228642a477bda255b32aadbce4\ <br> bda0b3f7e36c9da7 | v0.2.0 |
| hash   | Convert hex to SHA-2 256    | $ dtool hash -a sha2_256 0x616263 <br> 0xba7816bf8f01cfea414140de5dae2223b00361a3\ <br> 96177a9cb410ff61f20015ad | v0.2.0 |
| hash   | Convert hex to SHA-2 384    | $ dtool hash -a sha2_384 0x616263 <br> 0xcb00753f45a35e8bb5a03d699ac65007272c32ab\ <br> 0eded1631a8b605a43ff5bed8086072ba1e7cc23\ <br> 58baeca134c825a7 | v0.2.0 |
| hash   | Convert hex to SHA-2 512    | $ dtool hash -a sha2_512 0x616263 <br> 0xddaf35a193617abacc417349ae20413112e6fa4e\ <br> 89a97ea20a9eeee64b55d39a2192992a274fc1a8\ <br> 36ba3c23a3feebbd454d4423643ce80e2a9ac94f\ <br> a54ca49f | v0.2.0 |
| hash   | Convert hex to SHA-2 512 <br> truncate 224 | $ dtool hash -a sha2_512_224 0x616263 <br> 0x4634270f707b6a54daae7530460842e20e37ed26\ <br> 5ceee9a43e8924aa | v0.2.0 |
| hash   | Convert hex to SHA-2 512 <br> truncate 256 | $ dtool hash -a sha2_512_256 0x616263 <br> 0x53048e2681941ef99b2e29b76b4c7dabe4c2d0c6\ <br> 34fc6d46e0e2f13107e7af23 | v0.2.0 |
| hash   | Convert hex to SHA-3 224    | $ dtool hash -a sha3_224 0x616263 <br> 0xe642824c3f8cf24ad09234ee7d3c766fc9a3a516\ <br> 8d0c94ad73b46fdf | v0.2.0 |
| hash   | Convert hex to SHA-3 256    | $ dtool hash -a sha3_256 0x616263 <br> 0x3a985da74fe225b2045c172d6bd390bd855f086e\ <br> 3e9d525b46bfe24511431532 | v0.2.0 |
| hash   | Convert hex to SHA-3 384    | $ dtool hash -a sha3_384 0x616263 <br> 0xec01498288516fc926459f58e2c6ad8df9b473cb\ <br> 0fc08c2596da7cf0e49be4b298d88cea927ac7f5\ <br> 39f1edf228376d25 | v0.2.0 |
| hash   | Convert hex to SHA-3 512    | $ dtool hash -a sha3_512 0x616263 <br> 0xb751850b1a57168a5693cd924b6b096e08f62182\ <br> 7444f70d884f5d0240d2712e10e116e9192af3c9\ <br> 1a7ec57647e3934057340b4cf408d5a56592f827\ <br> 4eec53f0 | v0.2.0 |
| hash   | Convert hex to SHA-3 <br> keccak 224 | $ dtool hash -a sha3_k_224 0x616263 <br> 0xc30411768506ebe1c2871b1ee2e87d38df342317\ <br> 300a9b97a95ec6a8 | v0.2.0 |
| hash   | Convert hex to SHA-3 <br> keccak 256 | $ dtool hash -a sha3_k_256 0x616263 <br> 0x4e03657aea45a94fc7d47ba826c8d667c0d1e6e3\ <br> 3a64a036ec44f58fa12d6c45 | v0.2.0 |
| hash   | Convert hex to SHA-3 <br> keccak 384 | $ dtool hash -a sha3_k_384 0x616263 <br> 0xf7df1165f033337be098e7d288ad6a2f74409d7a\ <br> 60b49c36642218de161b1f99f8c681e4afaf31a3\ <br> 4db29fb763e3c28e | v0.2.0 |
| hash   | Convert hex to SHA-3 <br> keccak 512 | $ dtool hash -a sha3_k_512 0x616263 <br> 0x18587dc2ea106b9a1563e32b3312421ca164c7f1\ <br> f07bc922a9c83d77cea3a1e5d0c6991073902537\ <br> 2dc14ac9642629379540c17e2a65b19d77aa511a\ <br> 9d00bb96 | v0.2.0 |
| hash   | Convert hex to RIPEMD-160   | $ dtool hash -a ripemd_160 0x616263 <br> 0x8eb208f7e05d987a9b044a8e98c6b087f15a0bfc | v0.2.0 |
| s2u    | Convert UTF-8 String to <br> unicode | $ dtool s2u 💯 <br> \u1f4af <br> $ dtool s2u 💯 -f html <br> \&#x1f4af; <br> $ dtool s2u 💯 -f html_d <br> \&#128175; <br> $ dtool s2u 💯 -f rust <br> \u{1f4af} | v0.3.0 |
| u2s    | Convert unicode to <br> UTF-8 string | $ dtool u2s '\u1f4af'  <br> 💯 <br> $ dtool u2s '\&#x1f4af;'  <br> 💯 <br> $ dtool u2s '\&#128175;'  <br> 💯 <br> $ dtool u2s '\u{1f4af}'  <br> 💯 | v0.3.0 |

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
$ cat pic.jpg | dtool s2h | dtool h2b64
/9j/4AAQSkZJR...
```

calculate file md5
```
$ cat pic.jpg | dtool s2h | dtool hash -a md5
0x1884b72e23b0c93320bac6b050478ff4
```

## Installation
### Via cargo
```
$ cargo install dtool
```
