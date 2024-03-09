# csv-to-usv

Convert Comma Separated Values (CSV) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).

Syntax:

```sh
stdin | csv-to-usv | stdout
```

Example:

```sh
cat example.csv | csv-to-usv > example.usv
```


## Install

Install:

```sh
cargo install csv-to-usv
```

Link: [https://crates.io/crates/csv-to-usv](https://crates.io/crates/csv-to-usv)


## Example

Suppose file example.csv contains:

```usv
a,b,c
d,e,f
g,h,i
```

Run:

```sh
cat example.csv | csv-to-usv
```

Output:

```usv
a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞
```
