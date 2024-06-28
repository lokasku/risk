<br>
<br>
<p align="center"><img src="assets/risk.png" alt="Risk" width="150"></p>
<p align="center">Risk is a purely functional, strongly typed language.</p>

## Roadmap
- [x] Lexer
- [x] Parser
- [x] Semantic analysis
- [ ] Typechecker
- [ ] Codegen
- [ ] VM

## Features
* **Purely functional** : no side effects, pattern matching, parametric polymorphism and so on.
* **Portability** : Risk is compiled in bytecode and interpreted by our virtual machine, so it can be run anywhere.
* **Ligh** : as Risk is not a very advanced language, its source code is extremely light.

## Fib
```hs
fib :: Integer -> Integer
fib n = fib (n - 1) + fib (n - 2)
```

## How To Use
```bash
git clone https://github.com/lokasku/risk
cargo run <file.rk>
```
## Nix
```bash
nix run github:lokasku/risk <file.rk>
```
<br>

![GPL3](https://www.gnu.org/graphics/gplv3-127x51.png)
