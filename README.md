<br>
<br>
<div align="center">

<p><img src="assets/risk.png" alt="Risk" width="150"></p>

<p>Risk is a purely functional, strongly typed language.</p>

![GitHub Stars](https://img.shields.io/github/stars/lokasku/risk?style=for-the-badge&color=F3450D)
![GitHub Forks](https://img.shields.io/github/forks/lokasku/risk?style=for-the-badge&color=A65AC8)

</div>

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
f 0 = 0
f 1 = 1
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
