# dtool

dtool is a collection of development tools:

| Sub command   | Description   | Examples | 
| ------------- | :------------- | ------------ |
| h2s  | Convert hex to string   | $ dtool h2s 0x61626364 <br> abcd  |
| s2h  | Convert string to hex   | $ dtool h2s abcd <br> 0x61626364 |
| ts2d  | Convert timestamp to date   | $ dtool ts2d 10000 <br> 1970-01-01 10:46:40 |
| d2ts  | Convert date to timestamp   | $ dtool d2ts 1970-01-01 10:46:40 <br> 10000 |

## Installation
### Via cargo
```
$ cargo install dtool
```

### Via homebrew
```
$ brew install dtool
```
