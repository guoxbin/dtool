# Usage

## Table of Contents
- [Hex / UTF-8 string / binary / byte array conversion](#hex--utf-8-string--binary--byte-array-conversion)
- [Timestamp / date conversion](#timestamp--date-conversion)
- [Number 10/2/8/16 base conversion](#number-102816-base-conversion)
- [Hex / base58 conversion](#hex--base58-conversion)
- [Hex / base64 conversion](#hex--base64-conversion)
- [URL encode / decode](#url-encode--decode)
- [Number codec](#number-codec)
- [Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD, CRC, Blake2b, Blake3, SM3, Twox)](#hash-md5-sha-1-sha-2-sha-3-ripemd-crc-blake2b-blake3-sm3-twox)
- [UTF-8 string / unicode conversion](#utf-8-string--unicode-conversion)
- [HTML entity encode / decode](#html-entity-encode--decode)
- [Regex match](#regex-match)
- [Pbkdf2](#pbkdf2)
- [Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab, sarcasm)](#case-conversion-upper-lower-title-camel-pascal-snake-shouty-snake-kebab-sarcasm)
- [AES encrypt / decrypt](#aes-encrypt--decrypt)
- [ECDSA (Secp256k1, NIST P-256, NIST P-384, SM2)](#ecdsa-secp256k1-nist-p-256-nist-p-384-sm2)
- [SM4 encrypt / decrypt](#sm4-encrypt--decrypt)
- [EdDSA (Ed25519)](#eddsa-ed25519)
- [sr25519 signature](#sr25519-signature)

## Hex / UTF-8 string / binary / byte array conversion

|Sub command|                Desc                 |                        Example                        |
|-----------|-------------------------------------|-------------------------------------------------------|
|    h2s    |Convert hex to UTF-8 string<br>v0.1.0|            $ dtool h2s 0x61626364<br>abcd             |
|    s2h    |Convert UTF-8 string to hex<br>v0.1.0|            $ dtool s2h abcd<br>0x61626364             |
|    h2b    |   Convert hex to binary<br>v0.1.0   |            $ dtool h2b 0x61626364<br>abcd             |
|    b2h    |   Convert binary to hex<br>v0.1.0   |            $ dtool b2h abcd<br>0x61626364             |
|    h2a    | Convert hex to byte array<br>v0.7.0 |      $ dtool h2a 0x61626364<br>[97, 98, 99, 100]      |
|    a2h    | Convert byte array to hex<br>v0.7.0 |$ dtool a2h &#x27;[97, 98, 99, 100]&#x27;<br>0x61626364|


## Timestamp / date conversion

|Sub command|                           Desc                            |                               Example                                |
|-----------|-----------------------------------------------------------|----------------------------------------------------------------------|
|   ts2d    |            Convert timestamp to date<br>v0.1.0            |              $ dtool ts2d -z 0 0<br>1970-01-01 00:00:00              |
|   d2ts    |            Convert date to timestamp<br>v0.1.0            |        $ dtool d2ts -z 8 &#x27;1970-01-01 08:00:00&#x27;<br>0        |
|   d2ts    |Convert date to timestamp<br>Input rfc2822 format<br>v0.1.0|$ dtool d2ts &#x27;Mon, 23 Dec 2019 17:41:26 +0800&#x27;<br>1577094086|
|   d2ts    |Convert date to timestamp<br>Input rfc3339 format<br>v0.1.0|   $ dtool d2ts &#x27;2019-12-23T17:48:54+08:00&#x27;<br>1577094534   |
|    ts     |               Current timestamp<br>v0.12.0                |                      $ dtool ts <br>1647064300                       |


## Number 10/2/8/16 base conversion

|Sub command|                    Desc                     |                        Example                         |
|-----------|---------------------------------------------|--------------------------------------------------------|
|    ns     |  Number system<br>Input decimal<br>v0.1.0   | $ dtool ns 256<br>256<br>0b100000000<br>0o400<br>0x100 |
|    ns     |   Number system<br>Input octal<br>v0.1.0    |$ dtool ns 0o400<br>256<br>0b100000000<br>0o400<br>0x100|
|    ns     |  Number system<br>Output decimal<br>v0.1.0  |                $ dtool ns -d 256<br>256                |
|    ns     |  Number system<br>Output binary<br>v0.1.0   |            $ dtool ns -b 256<br>0b100000000            |
|    ns     |   Number system<br>Output octal<br>v0.1.0   |               $ dtool ns -o 256<br>0o400               |
|    ns     |Number system<br>Output hexadecimal<br>v0.1.0|               $ dtool ns -x 256<br>0x100               |


## Hex / base58 conversion

|Sub command|                Desc                 |                                             Example                                             |
|-----------|-------------------------------------|-------------------------------------------------------------------------------------------------|
|   h2b58   |   Convert hex to base58<br>v0.1.0   |   $ dtool h2b58 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a<br>12dvBhvPEPniQmBmgvj4qpJEodT7P   |
|  h2b58c   |Convert hex to base58 check<br>v0.1.0|$ dtool h2b58c 0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a<br>1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF|
|   b582h   |   Convert base58 to hex<br>v0.1.0   |   $ dtool b582h 12dvBhvPEPniQmBmgvj4qpJEodT7P<br>0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a   |
|  b58c2h   |Convert base58 check to hex<br>v0.1.0|$ dtool b58c2h 1Bi6zFVNtntP5MtDraNrAD7e469ifsQMwF<br>0x0075774f5d9963c021009a58d7d2d8e83771dd6c7a|


## Hex / base64 conversion

|Sub command|             Desc              |           Example            |
|-----------|-------------------------------|------------------------------|
|   h2b64   |Convert hex to base64<br>v0.1.0|$ dtool h2b64 0x616263<br>YWJj|
|   b642h   |Convert base64 to hex<br>v0.1.0|$ dtool b642h YWJj<br>0x616263|


## URL encode / decode

|Sub command|        Desc        |        Example        |
|-----------|--------------------|-----------------------|
|    ue     |URL encode<br>v0.1.0|$ dtool ue a+b<br>a%2Bb|
|    ud     |URL decode<br>v0.1.0|$ dtool ud a%2Bb<br>a+b|


## Number codec

|Sub command|               Desc               |                         Example                         |
|-----------|----------------------------------|---------------------------------------------------------|
|    ne     |  Number encode<br>u8<br>v0.1.0   |                $ dtool ne -tu8 1<br>0x01                |
|    ne     |  Number encode<br>u16<br>v0.1.0  |              $ dtool ne -tu16 1<br>0x0100               |
|    ne     |  Number encode<br>u32<br>v0.1.0  |            $ dtool ne -tu32 1<br>0x01000000             |
|    ne     |  Number encode<br>u64<br>v0.1.0  |        $ dtool ne -tu64 1<br>0x0100000000000000         |
|    ne     | Number encode<br>u128<br>v0.1.0  |$ dtool ne -tu128 1<br>0x01000000000000000000000000000000|
|    ne     |Number encode<br>Compact<br>v0.1.0|                $ dtool ne -tc 6<br>0x18                 |
|    ne     |Number encode<br>Compact<br>v0.1.0|              $ dtool ne -tc 251<br>0xed03               |
|    nd     |  Number decode<br>u8<br>v0.1.0   |                $ dtool nd -tu8 0x01<br>1                |
|    nd     |  Number decode<br>u16<br>v0.1.0  |              $ dtool nd -tu16 0x0100<br>1               |
|    nd     |  Number decode<br>u32<br>v0.1.0  |            $ dtool nd -tu32 0x01000000<br>1             |
|    nd     |  Number decode<br>u64<br>v0.1.0  |        $ dtool nd -tu64 0x0100000000000000<br>1         |
|    nd     | Number decode<br>u128<br>v0.1.0  |$ dtool nd -tu128 0x01000000000000000000000000000000<br>1|
|    nd     |Number decode<br>Compact<br>v0.1.0|                $ dtool nd -tc 0x18<br>6                 |
|    nd     |Number decode<br>Compact<br>v0.1.0|              $ dtool nd -tc 0xed03<br>251               |


## Hash (MD5, SHA-1, SHA-2, SHA-3, RIPEMD, CRC, Blake2b, Blake3, SM3, Twox)

|Sub command|                     Desc                      |                                                                                       Example                                                                                        |
|-----------|-----------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   hash    |         Hex to hash<br>MD5<br>v0.2.0          |                                                          $ dtool hash -a md5 0x616263<br>0x900150983cd24fb0d6963f7d28e17f72                                                          |
|   hash    |        Hex to hash<br>SHA-1<br>v0.2.0         |                                                     $ dtool hash -a sha1 0x616263<br>0xa9993e364706816aba3e25717850c26c9cd0d89d                                                      |
|   hash    |      Hex to hash<br>SHA-2 224<br>v0.2.0       |                                           $ dtool hash -a sha2_224 0x616263<br>0x23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7                                            |
|   hash    |      Hex to hash<br>SHA-2 256<br>v0.2.0       |                                    $ dtool hash -a sha2_256 0x616263<br>0xba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f2\\<br>0015ad                                     |
|   hash    |      Hex to hash<br>SHA-2 384<br>v0.2.0       |                    $ dtool hash -a sha2_384 0x616263<br>0xcb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43\\<br>ff5bed8086072ba1e7cc2358baeca134c825a7                     |
|   hash    |      Hex to hash<br>SHA-2 512<br>v0.2.0       | $ dtool hash -a sha2_512 0x616263<br>0xddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b\\<br>55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac9\\<br>4fa54ca49f  |
|   hash    |Hex to hash<br>SHA-2 512 truncate 224<br>v0.2.0|                                         $ dtool hash -a sha2_512_224 0x616263<br>0x4634270f707b6a54daae7530460842e20e37ed265ceee9a43e8924aa                                          |
|   hash    |Hex to hash<br>SHA-2 512 truncate 256<br>v0.2.0|                                  $ dtool hash -a sha2_512_256 0x616263<br>0x53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107\\<br>e7af23                                   |
|   hash    |      Hex to hash<br>SHA-3 224<br>v0.2.0       |                                           $ dtool hash -a sha3_224 0x616263<br>0xe642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf                                            |
|   hash    |      Hex to hash<br>SHA-3 256<br>v0.2.0       |                                    $ dtool hash -a sha3_256 0x616263<br>0x3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511\\<br>431532                                     |
|   hash    |      Hex to hash<br>SHA-3 384<br>v0.2.0       |                    $ dtool hash -a sha3_384 0x616263<br>0xec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e4\\<br>9be4b298d88cea927ac7f539f1edf228376d25                     |
|   hash    |      Hex to hash<br>SHA-3 512<br>v0.2.0       | $ dtool hash -a sha3_512 0x616263<br>0xb751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240\\<br>d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8\\<br>274eec53f0  |
|   hash    |   Hex to hash<br>SHA-3 keccak 224<br>v0.2.0   |                                          $ dtool hash -a sha3_k_224 0x616263<br>0xc30411768506ebe1c2871b1ee2e87d38df342317300a9b97a95ec6a8                                           |
|   hash    |   Hex to hash<br>SHA-3 keccak 256<br>v0.2.0   |                                   $ dtool hash -a sha3_k_256 0x616263<br>0x4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa1\\<br>2d6c45                                    |
|   hash    |   Hex to hash<br>SHA-3 keccak 384<br>v0.2.0   |                   $ dtool hash -a sha3_k_384 0x616263<br>0xf7df1165f033337be098e7d288ad6a2f74409d7a60b49c36642218de16\\<br>1b1f99f8c681e4afaf31a34db29fb763e3c28e                    |
|   hash    |   Hex to hash<br>SHA-3 keccak 512<br>v0.2.0   |$ dtool hash -a sha3_k_512 0x616263<br>0x18587dc2ea106b9a1563e32b3312421ca164c7f1f07bc922a9c83d77ce\\<br>a3a1e5d0c69910739025372dc14ac9642629379540c17e2a65b19d77aa51\\<br>1a9d00bb96 |
|   hash    |      Hex to hash<br>RIPEMD-160<br>v0.2.0      |                                                  $ dtool hash -a ripemd_160 0x616263<br>0x8eb208f7e05d987a9b044a8e98c6b087f15a0bfc                                                   |
|   hash    |        Hex to hash<br>CRC32<br>v0.5.0         |                                                                    $ dtool hash -a crc_32 0x616263<br>0x352441c2                                                                     |
|   hash    |     Hex to hash<br>Blake2b 160<br>v0.5.0      |                                                  $ dtool hash -a blake2b_160 0x616263<br>0x384264f676f39536840523f284921cdc68b6846b                                                  |
|   hash    |     Hex to hash<br>Blake2b 256<br>v0.5.0      |                                   $ dtool hash -a blake2b_256 0x616263<br>0xbddd813c634239723171ef3fee98579b94964e3bb1cb3e427262c8c068\\<br>d52319                                   |
|   hash    |     Hex to hash<br>Blake2b 384<br>v0.5.0      |                   $ dtool hash -a blake2b_384 0x616263<br>0x6f56a82c8e7ef526dfe182eb5212f7db9df1317e57815dbda46083fc30\\<br>f54ee6c66ba83be64b302d7cba6ce15bb556f4                   |
|   hash    |     Hex to hash<br>Blake2b 512<br>v0.5.0      |$ dtool hash -a blake2b_512 0x616263<br>0xba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdb\\<br>ffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386\\<br>edd4009923|
|   hash    |     Hex to hash<br>Blake3<br>v0.5.0           |                                    $ dtool hash -a blake3 0x616263<br>0x6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85                                                                                 |
|   hash    |         Hex to hash<br>SM3<br>v0.7.0          |                                       $ dtool hash -a sm3 0x616263<br>0x66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f\\<br>4ba8e0                                       |
|   hash    |        Hex to hash<br>TwoX<br>v0.10.0         |                                                               $ dtool hash -a twox -s 1 0x616263<br>0x0889329981caa9be                                                               |


## UTF-8 string / unicode conversion

|Sub command|                          Desc                          |                           Example                           |
|-----------|--------------------------------------------------------|-------------------------------------------------------------|
|    s2u    |  UTF-8 string to unicode<br>Default format<br>v0.3.0   |               $ dtool s2u abc<br>\u61\u62\u63               |
|    s2u    |    UTF-8 string to unicode<br>HTML format<br>v0.3.0    |  $ dtool s2u -f html abc<br>&amp;#x61;&amp;#x62;&amp;#x63;  |
|    s2u    |UTF-8 string to unicode<br>HTML decimal format<br>v0.3.0|  $ dtool s2u -f html_d abc<br>&amp;#97;&amp;#98;&amp;#99;   |
|    s2u    |    UTF-8 string to unicode<br>RUST format<br>v0.3.0    |        $ dtool s2u -f rust abc<br>\u{61}\u{62}\u{63}        |
|    s2u    |       UTF-8 string to unicode<br>Emoji<br>v0.3.0       |                  $ dtool s2u 💯<br>\u1f4af                   |
|    u2s    |Unicode to UTF-8 string<br>From default format<br>v0.3.0|         $ dtool u2s &#x27;\u61\u62\u63&#x27;<br>abc         |
|    u2s    |    Unicode to UTF-8 string<br>HTML format<br>v0.3.0    |$ dtool u2s &#x27;&amp;#x61;&amp;#x62;&amp;#x63;&#x27;<br>abc|
|    u2s    |Unicode to UTF-8 string<br>HTML decimal format<br>v0.3.0| $ dtool u2s &#x27;&amp;#97;&amp;#98;&amp;#99;&#x27;<br>abc  |
|    u2s    |    Unicode to UTF-8 string<br>RUST format<br>v0.3.0    |      $ dtool u2s &#x27;\u{61}\u{62}\u{63}&#x27;<br>abc      |
|    u2s    |       Unicode to UTF-8 string<br>Emoji<br>v0.3.0       |            $ dtool u2s &#x27;\u1f4af&#x27;<br>💯             |


## HTML entity encode / decode

|Sub command|            Desc            |                       Example                       |
|-----------|----------------------------|-----------------------------------------------------|
|    he     |HTML entity encode<br>v0.4.0|$ dtool he &#x27;&lt;b&gt;&#x27;<br>&amp;lt;b&amp;gt;|
|    hd     |HTML entity decode<br>v0.4.0|$ dtool hd &#x27;&amp;lt;b&amp;gt;&#x27;<br>&lt;b&gt;|


## Regex match

|Sub command|        Desc         |                                                            Example                                                             |
|-----------|---------------------|--------------------------------------------------------------------------------------------------------------------------------|
|    re     |Regex match<br>v0.4.0|$ dtool re -p &#x27;a(.)c&#x27; abcadc<br>abc<br>&nbsp;&nbsp;&nbsp;&nbsp;group#1: b<br>adc<br>&nbsp;&nbsp;&nbsp;&nbsp;group#1: d|


## Pbkdf2

|Sub command|      Desc      |                                                                Example                                                                |
|-----------|----------------|---------------------------------------------------------------------------------------------------------------------------------------|
|  pbkdf2   |Pbkdf2<br>v0.5.0|$ dtool pbkdf2 -a sha2_256 -s 0x646566 -i 2 -l 256 0x616263<br>0x51a30556d0d133d859d3f3da86f861b7b12546c4f9a193ebb374397467\\<br>872514|


## Case conversion (upper, lower, title, camel, pascal, snake, shouty snake, kebab, sarcasm)

|Sub command|                     Desc                     |                                                                      Example                                                                      |
|-----------|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
|   case    |   Case conversion<br>Upper case<br>v0.5.0    |                                             $ dtool case -t upper &#x27;good tool&#x27;<br>GOOD TOOL                                              |
|   case    |   Case conversion<br>Lower case<br>v0.5.0    |                                             $ dtool case -t lower &#x27;GOOD TOOL&#x27;<br>good tool                                              |
|   case    |   Case conversion<br>Title case<br>v0.5.0    |                                             $ dtool case -t title &#x27;GOOD TOOL&#x27;<br>Good Tool                                              |
|   case    |   Case conversion<br>Camel case<br>v0.5.0    |                                              $ dtool case -t camel &#x27;GOOD TOOL&#x27;<br>goodTool                                              |
|   case    |   Case conversion<br>Pascal case<br>v0.5.0   |                                             $ dtool case -t pascal &#x27;GOOD TOOL&#x27;<br>GoodTool                                              |
|   case    |   Case conversion<br>Snake case<br>v0.5.0    |                                                    $ dtool case -t snake GoodTool<br>good_tool                                                    |
|   case    |Case conversion<br>Shouty snake case<br>v0.5.0|                                                $ dtool case -t shouty_snake GoodTool<br>GOOD_TOOL                                                 |
|   case    |   Case conversion<br>Kebab case<br>v0.5.0    |                                                    $ dtool case -t kebab GoodTool<br>good-tool                                                    |
|   case    |  Case conversion<br>Sarcasm case<br>v0.9.0   |                                                  $ dtool case -t sarcasm good tool<br>gOoD tOoL                                                   |
|   case    |   Case conversion<br>All cases<br>v0.11.0    |$ dtool case good tool<br>GOOD TOOL<br>good tool<br>Good Tool<br>goodTool<br>GoodTool<br>good_tool<br>GO\\<br>OD_TOOL<br>good-tool<br>gOoD tOoL<br>|


## AES encrypt / decrypt

|Sub command|                  Desc                  |                                                                                                     Example                                                                                                     |
|-----------|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  aes_enc  |AES encrypt<br>KeySize 128 ECB<br>v0.6.0|                                     $ dtool aes_enc -k 01010101010101010101010101010101 -m ecb 0\\<br>x616263646162636461626364616263<br>0xe89c98329f3e8b6da3e714fbba2be6d1                                     |
|  aes_enc  |AES encrypt<br>KeySize 192 ECB<br>v0.6.0|                             $ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>1010101 -m ecb 0x616263646162636461626364616263<br>0x88fe17738e31914c9166f9b101d1b028                             |
|  aes_enc  |AES encrypt<br>KeySize 256 ECB<br>v0.6.0|                  $ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -m ecb 0x616263646162636461626364616\\<br>263<br>0x3e6bcc9d26c494b1c6971316020acd3a                  |
|  aes_enc  |AES encrypt<br>KeySize 128 CBC<br>v0.6.0|                $ dtool aes_enc -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m cbc 0x61626364616263646162636\\<br>4616263<br>0x350678b99c37ab5f68f560551e960572                |
|  aes_enc  |AES encrypt<br>KeySize 192 CBC<br>v0.6.0|        $ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>1010101 -i 03030303030303030303030303030303 -m cbc 0x6162636\\<br>46162636461626364616263<br>0xbbc8ff4de1a197e67a5f8f4d7a35f9a0        |
|  aes_enc  |AES encrypt<br>KeySize 256 CBC<br>v0.6.0|$ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -i 03030303030303030303030303030303 \\<br>-m cbc 0x616263646162636461626364616263<br>0x3309a7511f007e993676a90a06391d28|
|  aes_enc  |AES encrypt<br>KeySize 128 CTR<br>v0.6.0|                                            $ dtool aes_enc -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m ctr 0x616263<br>0x075e64                                            |
|  aes_enc  |AES encrypt<br>KeySize 192 CTR<br>v0.6.0|                                    $ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>1010101 -i 03030303030303030303030303030303 -m ctr 0x616263<br>0xbad37a                                    |
|  aes_enc  |AES encrypt<br>KeySize 256 CTR<br>v0.6.0|                         $ dtool aes_enc -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -i 03030303030303030303030303030303 \\<br>-m ctr 0x616263<br>0x9e5062                         |
|  aes_dec  |AES decrypt<br>KeySize 128 ECB<br>v0.6.0|                                     $ dtool aes_dec -k 01010101010101010101010101010101 -m ecb 0\\<br>xe89c98329f3e8b6da3e714fbba2be6d1<br>0x616263646162636461626364616263                                     |
|  aes_dec  |AES decrypt<br>KeySize 192 ECB<br>v0.6.0|                             $ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>1010101 -m ecb 0x88fe17738e31914c9166f9b101d1b028<br>0x616263646162636461626364616263                             |
|  aes_dec  |AES decrypt<br>KeySize 256 ECB<br>v0.6.0|                  $ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -m ecb 0x3e6bcc9d26c494b1c6971316020\\<br>acd3a<br>0x616263646162636461626364616263                  |
|  aes_dec  |AES decrypt<br>KeySize 128 CBC<br>v0.6.0|                $ dtool aes_dec -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m cbc 0x350678b99c37ab5f68f5605\\<br>51e960572<br>0x616263646162636461626364616263                |
|  aes_dec  |AES decrypt<br>KeySize 192 CBC<br>v0.6.0|        $ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>1010101 -i 03030303030303030303030303030303 -m cbc 0xbbc8ff4\\<br>de1a197e67a5f8f4d7a35f9a0<br>0x616263646162636461626364616263        |
|  aes_dec  |AES decrypt<br>KeySize 256 CBC<br>v0.6.0|$ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -i 03030303030303030303030303030303 \\<br>-m cbc 0x3309a7511f007e993676a90a06391d28<br>0x616263646162636461626364616263|
|  aes_dec  |AES decrypt<br>KeySize 128 CTR<br>v0.6.0|                                            $ dtool aes_dec -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m ctr 0x075e64<br>0x616263                                            |
|  aes_dec  |AES decrypt<br>KeySize 192 CTR<br>v0.6.0|                                    $ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>1010101 -i 03030303030303030303030303030303 -m ctr 0xbad37a<br>0x616263                                    |
|  aes_dec  |AES decrypt<br>KeySize 256 CTR<br>v0.6.0|                         $ dtool aes_dec -k 01010101010101010101010101010101010101010\\<br>10101010101010101010101 -i 03030303030303030303030303030303 \\<br>-m ctr 0x9e5062<br>0x616263                         |


## ECDSA (Secp256k1, NIST P-256, NIST P-384, SM2)

|Sub command|                                        Desc                                        |                                                                                                                                                                                                                                             Example                                                                                                                                                                                                                                              |
|-----------|------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   ec_gk   |Elliptic-curve generate key pair (Secret<br> key, Public key)<br>Secp256k1<br>v0.7.0|                                                                                                                                                     $ dtool ec_gk -c secp256k1 -C<br>(0x9cbe9cd5d7759ca46296f64e3e8211ef5ccaf86b5cb7169711554d1ed\\<br>2ed68ca, 0x0379ce37925295f3103855da38ee2bf0e06a60ec9d86806d0\\<br>efd2de3649a74b40d)                                                                                                                                                      |
|   ec_gk   |  Elliptic-curve generate key pair (Secret<br> key, Public key)<br>P-256<br>v0.7.0  |                                                                                                                      $ dtool ec_gk -c p256<br>(0xf0b3b41add2d79932cdf2a4ba083c16e72647ddcd8718e2187d1567ed\\<br>5a611c9, 0x045c79019e39199effa07576de6e3745fa1dba402854314ae\\<br>f05790e9e827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57\\<br>cb53e16e2839bdbbee630)                                                                                                                       |
|   ec_gk   |   Elliptic-curve generate key pair (Secret<br> key, Public key)<br>SM2<br>v0.7.0   |                                                                                                                       $ dtool ec_gk -c sm2<br>(0x80a61373e34f7215feceb8dd06bb3731ea362ff5355a7226d4e12d076\\<br>a7eb588, 0x044b2dd8bf6dbbfb14db3e4d17bd7a3e8758eb4232049bec9\\<br>31d1038f4afaae46ac3c771f929bbf35a28b0363789fb19127cea3318f4c\\<br>8902a0034ca5f1b7667d1)                                                                                                                       |
|  ec_sign  |                     Elliptic-curve sign<br>Secp256k1<br>v0.7.0                     |                                                                                                               $ dtool ec_sign -c secp256k1 -s 0x9cb4f775e9b67118242cea1528\\<br>5555c287a7e3d2f86ba238c1fe87284b898e9a 0x616263<br>0x7c77b65a27984b0e124a0ae2eec6bbf2b338a5c999b943abda576108f9\\<br>2e95364b0b983da055493c87fd138fe5673992b2a48ef85d9ad30c98fc1a\\<br>fcc5fc7bc0                                                                                                                |
|  ec_sign  |                       Elliptic-curve sign<br>P-256<br>v0.7.0                       |                                                                                                                  $ dtool ec_sign -c p256 -s 0xf0b3b41add2d79932cdf2a4ba083c16\\<br>e72647ddcd8718e2187d1567ed5a611c9 0x616263<br>0x495f62f272440bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2\\<br>111cabb91a79328ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b\\<br>923a81cc69                                                                                                                  |
|  ec_sign  |                        Elliptic-curve sign<br>SM2<br>v0.7.0                        |                                                                                                                  $ dtool ec_sign -c sm2 -s 0x80a61373e34f7215feceb8dd06bb3731\\<br>ea362ff5355a7226d4e12d076a7eb588 0x616263<br>0x0a4d089d3177234ed34aa7f30c6a7a7954539f68825bedbe82be65aefd\\<br>b733c921207be31b8071bbfd5c99044ebde49d3c38e9972063b844f65f4a\\<br>cfc7d6dff2                                                                                                                   |
| ec_verify |                    Elliptic-curve verify<br>Secp256k1<br>v0.7.0                    |                                                                                                      $ dtool ec_verify -c secp256k1 -p 0x03391aa7238b79e1aad1e038\\<br>c95306171a8ac7499357dc99586f96c5f3b9618d60 -S 0x7c77b65a2798\\<br>4b0e124a0ae2eec6bbf2b338a5c999b943abda576108f92e95364b0b983d\\<br>a055493c87fd138fe5673992b2a48ef85d9ad30c98fc1afcc5fc7bc0 0x6\\<br>16263<br>true                                                                                                       |
| ec_verify |                      Elliptic-curve verify<br>P-256<br>v0.7.0                      |                                                                      $ dtool ec_verify -c p256 -p 0x045c79019e39199effa07576de6e3\\<br>745fa1dba402854314aef05790e9e827cf7782ac5feb26e28039f94d7307\\<br>8c57b5f29be14ef9da57cb53e16e2839bdbbee630 -S 0x495f62f272440\\<br>bd0621d27e97d60c57a0cdaef1cc2434c454eae833bb2111cabb91a79328\\<br>ee766f720a888b14e0f6037eb8a397dcd9bc9f4c18b9b923a81cc69 0x61\\<br>6263<br>true                                                                      |
| ec_verify |                      Elliptic-curve verify<br>P-384<br>v0.7.0                      |$ dtool ec_verify -c p384 -p 0x044978c6c7be1a5c5194983a945d2\\<br>d8c81ae4b421dd89d12c6dd1756d2387fa2601993657eeb93d289a57625a\\<br>70c2830db5f06f988a3e4549e26e8b6d27c7f1e6e8949d6ce5bf3f88a0f5\\<br>eebaa14499d4379bc81cca6e9ff17d18b8efb370fffe3 -S 0xa0d387bc5\\<br>d5de4979750f531f337fd1d04384ab4a9d251a18852c1ce1a16e2e46a277\\<br>8764d0b3ee090babbc5092ea57a108ddabf9a9fcf8efaad7c0862da2bedd\\<br>de806745c0c3972d738c416d55cfde19b85e39ab54151c87b537c4df7d17\\<br>7ff 0x616263<br>true|
| ec_verify |                       Elliptic-curve verify<br>SM2<br>v0.7.0                       |                                                                      $ dtool ec_verify -c sm2 -p 0x044b2dd8bf6dbbfb14db3e4d17bd7a\\<br>3e8758eb4232049bec931d1038f4afaae46ac3c771f929bbf35a28b03637\\<br>89fb19127cea3318f4c8902a0034ca5f1b7667d1 -S 0x0a4d089d317723\\<br>4ed34aa7f30c6a7a7954539f68825bedbe82be65aefdb733c921207be31b\\<br>8071bbfd5c99044ebde49d3c38e9972063b844f65f4acfc7d6dff2 0x616\\<br>263<br>true                                                                       |
|   ec_pk   |             Elliptic-curve calculate public key<br>Secp256k1<br>v0.7.0             |                                                                                                                    $ dtool ec_pk -c secp256k1 -s 0x9cb4f775e9b67118242cea152855\\<br>55c287a7e3d2f86ba238c1fe87284b898e9a<br>0x04391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3\\<br>b9618d6035af9529d80a85ebecb1120d1cfaf1591b7c686907b0a3d18858\\<br>a95e86976747                                                                                                                    |
|   ec_pk   |  Elliptic-curve calculate public key<br>Secp256k1 Compressed public key<br>v0.7.0  |                                                                                                                                                     $ dtool ec_pk -c secp256k1 -s 0x9cb4f775e9b67118242cea152855\\<br>55c287a7e3d2f86ba238c1fe87284b898e9a -C<br>0x03391aa7238b79e1aad1e038c95306171a8ac7499357dc99586f96c5f3\\<br>b9618d60                                                                                                                                                      |
|   ec_pk   |               Elliptic-curve calculate public key<br>P-256<br>v0.7.0               |                                                                                                                      $ dtool ec_pk -c p256 -s 0xf0b3b41add2d79932cdf2a4ba083c16e7\\<br>2647ddcd8718e2187d1567ed5a611c9<br>0x045c79019e39199effa07576de6e3745fa1dba402854314aef05790e9e\\<br>827cf7782ac5feb26e28039f94d73078c57b5f29be14ef9da57cb53e16e2\\<br>839bdbbee630                                                                                                                       |
|   ec_pk   |                Elliptic-curve calculate public key<br>SM2<br>v0.7.0                |                                                                                                                       $ dtool ec_pk -c sm2 -s 0x80a61373e34f7215feceb8dd06bb3731ea\\<br>362ff5355a7226d4e12d076a7eb588<br>0x044b2dd8bf6dbbfb14db3e4d17bd7a3e8758eb4232049bec931d1038f4\\<br>afaae46ac3c771f929bbf35a28b0363789fb19127cea3318f4c8902a0034\\<br>ca5f1b7667d1                                                                                                                       |


## SM4 encrypt / decrypt

|Sub command|            Desc            |                                                         Example                                                         |
|-----------|----------------------------|-------------------------------------------------------------------------------------------------------------------------|
|  sm4_enc  |SM4 encrypt<br>CTR<br>v0.6.0|$ dtool sm4_enc -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m ctr 0x616263<br>0x8cd7ea|
|  sm4_dec  |SM4 decrypt<br>CTR<br>v0.7.0|$ dtool sm4_dec -k 01010101010101010101010101010101 -i 03030\\<br>303030303030303030303030303 -m ctr 0x8cd7ea<br>0x616263|


## EdDSA (Ed25519)

|Sub command|                               Desc                                |                                                                                                                            Example                                                                                                                             |
|-----------|-------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   ed_gk   |EdDSA generate key pair (Mini secret key<br>, Public key)<br>v0.8.0|                                             $ dtool ed_gk<br>(0xb850164d1feec8698acca329947c9885bd1d94034d2fbbe6080598adb\\<br>e15b298, 0x892c89a4cd631d08da314607223814775604535a05f50e959\\<br>d21209d01740eba)                                              |
|  ed_sign  |            EdDSA sign<br>Use mini secret key<br>v0.8.0            |     $ dtool ed_sign -m 0xb850164d1feec8698acca329947c9885bd1d940\\<br>34d2fbbe6080598adbe15b298 0x616263<br>0x52131a69ebb236703de0c3589689202eebd1d16c40990c3ad8b3582631\\<br>a7a267db745dbb9156d8626187e40f42f6cfe884b6d3ce0cdc04603afeed\\<br>089703ac0e     |
| ed_verify |                      EdDSA verify<br>v0.8.0                       |$ dtool ed_verify -p 0x892c89a4cd631d08da3146072238147756045\\<br>35a05f50e959d21209d01740eba -S 0x52131a69ebb236703de0c358968\\<br>9202eebd1d16c40990c3ad8b3582631a7a267db745dbb9156d8626187e40\\<br>f42f6cfe884b6d3ce0cdc04603afeed089703ac0e 0x616263<br>true|
|   ed_pk   |    EdDSA calculate public key<br>Use mini secret key<br>v0.8.0    |                                             $ dtool ed_pk -m 0xb850164d1feec8698acca329947c9885bd1d94034\\<br>d2fbbe6080598adbe15b298<br>0x892c89a4cd631d08da314607223814775604535a05f50e959d21209d01\\<br>740eba                                              |


## sr25519 signature

|Sub command|                                Desc                                 |                                                                                                                                                          Example                                                                                                                                                           |
|-----------|---------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   sr_gk   |sr25519 generate key pair (Mini secret k<br>ey, Public key)<br>v0.8.0|                                                                           $ dtool sr_gk<br>(0xc243239f434f7a4b0ab8d4600537001e6479c807c3d3623f99c8ad9f2\\<br>a588837, 0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50\\<br>ae4cd57109e9d6c)                                                                            |
|  sr_sign  |            sr25519 sign<br>Use mini secret key<br>v0.8.0            |                                   $ dtool sr_sign -m 0xc243239f434f7a4b0ab8d4600537001e6479c80\\<br>7c3d3623f99c8ad9f2a588837 0x616263<br>0xced639526bb840107f33b7e6588219bae8657707f0537dce9969338748\\<br>673d54b92e0efba5477a1494696e5cf3f5e7a40f03271b1ef2e2030ef60d\\<br>6be1caa784                                   |
|  sr_sign  |              sr25519 sign<br>Use secret key<br>v0.8.0               |$ dtool sr_sign -s 0xb0f4e5710d79bf6a46391e1c6e50a883af76763\\<br>6d55bcad178aa7ec7f1aa750dee6c27bbe26656a29f06ea1612461a86a19\\<br>0db16b31ddd6b78354fb6ba57bf7d 0x616263<br>0xced639526bb840107f33b7e6588219bae8657707f0537dce9969338748\\<br>673d54b92e0efba5477a1494696e5cf3f5e7a40f03271b1ef2e2030ef60d\\<br>6be1caa784|
| sr_verify |                      sr25519 verify<br>v0.8.0                       |                              $ dtool sr_verify -p 0x6a8ee649b31efe7aabd8d5af58f85c60f12c4\\<br>8f8aa880cb50ae4cd57109e9d6c -S 0xced639526bb840107f33b7e6588\\<br>219bae8657707f0537dce9969338748673d54b92e0efba5477a1494696e5\\<br>cf3f5e7a40f03271b1ef2e2030ef60d6be1caa784 0x616263<br>true                              |
|   sr_sk   |   sr25519 calculate secret key from mini s<br>ecret key<br>v0.8.0   |                                        $ dtool sr_sk -m 0xc243239f434f7a4b0ab8d4600537001e6479c807c\\<br>3d3623f99c8ad9f2a588837<br>0xb0f4e5710d79bf6a46391e1c6e50a883af767636d55bcad178aa7ec7f1\\<br>aa750dee6c27bbe26656a29f06ea1612461a86a190db16b31ddd6b78354f\\<br>b6ba57bf7d                                         |
|   sr_pk   |    sr25519 calculate public key<br>Use mini secret key<br>v0.8.0    |                                                                           $ dtool sr_pk -m 0xc243239f434f7a4b0ab8d4600537001e6479c807c\\<br>3d3623f99c8ad9f2a588837<br>0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd5710\\<br>9e9d6c                                                                            |
|   sr_pk   |      sr25519 calculate public key<br>Use secret key<br>v0.8.0       |                                        $ dtool sr_pk -s 0xb0f4e5710d79bf6a46391e1c6e50a883af767636d\\<br>55bcad178aa7ec7f1aa750dee6c27bbe26656a29f06ea1612461a86a190d\\<br>b16b31ddd6b78354fb6ba57bf7d<br>0x6a8ee649b31efe7aabd8d5af58f85c60f12c48f8aa880cb50ae4cd5710\\<br>9e9d6c                                         |


