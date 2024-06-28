As the language is still under development, many bugs persist in a number of places, particularly and almost exclusively in the parser. In the meantime, here is a brief presentation of the language's syntax :

## Function
`f args = expr`
```hs
g :: Bool -> Bool -> String
g a b = if (a && b == True) then "Yes" else "No"
```

## `Match`
`match expr with (case -> ret) ...`
```hs
e = match 2 * 4 with
            (0 -> 'a')
            (2 -> 'b')
            (4 -> 'c')
            (8 -> 'd')
            (_ -> 'z')
```
## `Let`
`let x = e; ...; in expr`
```hs
e = let x = 2; y = 4; in x * y
x = e / 4
```
## `Type`
`type X args = (A args) ...`
```ocaml
type X a b = (A a) (B b)
```
## Expression
```hs
-- String
"hello, world"

-- Char
'a' 'b' 'z' ')'

-- Integer
2 4 2048

-- Float
4.28 64.11

-- Bool
True
False

-- Lambda
\x -> x + 1
e = \x -> \y -> x + y

-- List
[2, 4, 8, 16]
["to", "be", "or", "not", "to", "be"]
[True, False]

-- Tuple
(1, 2, 3)
('a', 'b', 'c')
(True, False)

-- Condition
if True then True else False
```
## Operators
### Bool
`&&` `||` `==` `/=`
### Numbers
`+` `-` `*` `/` `%` `^` `>` `<` `>=` `<=`