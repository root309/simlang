### about
Small interpreter language.

Under developing.

Example code will work.

### repl
```
cargo run
```
```sh
λ 10 + 10;
```
```sh
λ nyarn = 10; yarn = 10; nyarn + yarn;
```

### source file
cargo run filename


#### examples
- function
```
function add(x, y) {
  return x + y;
};

add(100, 200);
```

- if
```
nyarn = 30;
yarn = 20;

if (nyarn < yarn) { 10; }
else { 5; }
```

- while
```
i = 0;
sum = 0;

while(i < 100) {
  sum = sum + 1;
  i = i + 1;
};

sum;
```
