It is still under development and is not nearly as functional as a programming language.

### repl
```
cargo run
```
```sh
λ 10 + 10;
```
### source file
cargo run filename.sim


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
