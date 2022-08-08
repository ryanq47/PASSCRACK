# PASSCRACK
A very basic, single-threaded Rust based password cracker. Supports basic hashes, such as MD5, MD4, SHA3-*, and a variety of others

## <b>Syntax</b>

./pass_crack </br>
    -w: Wordlist (ex. /usr/share/wordlists/rockyou) </br>
    -t: Type of hash (ex. sha2) </br>
    -h: Raw hash (Currently hash files are not supported) </br>
    
    
#### <b>Example Command:</b>
    ./pass_crack -w rockyou.txt -t sha2 -h ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193

