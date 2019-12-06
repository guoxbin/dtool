# dtool

dtool is a collection of development tools:

| Sub command   | Description   | Examples | 
| ------------- | :------------- | ------------ |
| h2s    | Convert hex to string       | $ dtool h2s 0x61626364 <br> abcd  |
| s2h    | Convert string to hex       | $ dtool h2s abcd <br> 0x61626364 |
| ts2d   | Convert timestamp to date   | $ dtool ts2d 10000 <br> 1970-01-01 10:46:40 |
| d2ts   | Convert date to timestamp   | $ dtool d2ts 1970-01-01 10:46:40 <br> 10000 |
| d2ts   | Number system               | $ dtool ns 256 <br> 256 <br> 0b100000000 <br> 0o400 <br> 0x100 <br> |
| h2b58  | Convert hex to base58       | $ dtool h2b58 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a <br> 12dvBhvPEPniQmBmgvj4qpJEodT7P |
| h2b58c | Convert hex to base58 check | $ dtool h2b58c 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a <br> 1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF |
| b582h  | Convert base58 to hex       | $ dtool b582h 12dvBhvPEPniQmBmgvj4qpJEodT7P <br> 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a |
| b58c2h | Convert base58 check to hex | $ dtool b58c2h 1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF <br> 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a |
| h2b64  | Convert hex to base64       | $ dtool h2b64 0x616263 <br> YWJj |
| b642h  | Convert base64 to hex       | $ dtool b642h YWJj <br> 0x616263 |
| ue     | URL encode                  | $ dtool ue a+b <br> a%2Bb |
| ud     | URL decode                  | $ dtool ue a%2Bb <br> a+b |
| ne     | Number encode               | $ dtool ne -tu8 1 <br> 0x01 <br> $ dtool ne -tu16 1 <br> 0x0100 <br> $ dtool ne -tu32 1 <br> 0x01000000 <br> $ dtool ne -tu64 1 <br> 0x0100000000000000 <br> $ dtool ne -tu128 1 <br> 0x01000000000000000000000000000000 <br> $ dtool ne -tc 6 <br> 0x18 <br> dtool ne -tc 251 <br> 0xed03 |
| nd     | Number decode               | $ $ dtool nd -tu8 0x01 <br> 1 <br> $ dtool nd -tu16 0x0100 <br> 1 <br> $ dtool nd -tu32 0x01000000 <br> 1 <br> $ dtool nd -tu64 0x0100000000000000 <br> 1 <br> $ dtool nd -tu128 0x01000000000000000000000000000000 <br> 1 <br> $ dtool nd -tc 0x18 <br> 6 <br> $ dtool nd -tc 0xed03 <br> 251 |

## Tips

### pipe 
you can convert a string to base64 by a pipe
```
$ echo -n abc | dtool s2h | dtool h2b64
YWJj
```

you can convert a encoded timestamp to date
```
$ echo -n 2c28e75d | dtool nd -tu32 | dtool ts2d
2019-12-04 11:29:48
```

## Installation
### Via cargo
```
$ cargo install dtool
```

### Via homebrew
```
$ brew install dtool
```
