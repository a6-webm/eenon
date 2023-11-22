# eenon
### An IOU balancer
Given a .csv file formated something like this:
```
Timestamp | Spender  | Who is it for?        | What you bought | Cost ($)
18th June | Bob      | Cameron, Jemimah, Bob | Pizza           | 27
19th June | Josefine | Bob                   | Ben & Jerry's   | 4.48
```
eenon will tell you this:
```
Bob owes Josefine 4.48
Jemimah owes Bob 9.00
Cameron owes Bob 9.00
```

(If you really want to know why it's called "eenon", then you need to play more Rhythm Heaven.)

# Running the code
Install `cargo`, download the code, and run:
```
cargo run -- <FILE>
```
 where `<FILE>` is the path to your .csv file.
