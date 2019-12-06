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

## Installation
### Via cargo
```
$ cargo install dtool
```

### Via homebrew
```
$ brew install dtool
```
