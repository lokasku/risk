`let` `in`
`if` `then` `else`
`match` `with` + `|`
`type` + `|`
`alias`

`+` `-` `/` `%` `<` `>` `>=` `<=` `,` `->` `=` `==` `^` `:` `\` `&&` `||` (``` ```) `_`
                                                             ^
                                                           lambda
`(` `)` `[` `]`

`Float` `Double`
`Integer`
`Char`
`Bool` : `True` and `False`
`(x, y)`
`[x, y]`
Indentifier as it's parametric polymorphism

If the name consists of letters, it is a function which requires backticks to be called infix => Prefix by default
If the name consists of symbols, it is an operator which requires parentheses to be called prefix. => Infix by default

(##) :: Integer -> Integer -> Integer
1. a ## b = a + b
2. (##) a b = a + b

foo :: Integer -> Integer -> Integer
1. foo a b = a + b
2. a `foo` b = a + b


type X a b = A a | B b

@f :: c -> b -> Maybe (c) -> Integer
decl f x:xs Just (x) = let a = 1; b = 2; in a

@a :: Integer
decl a = match 2 with
            | 0 -> 'a'
            | 1 -> 'b'
            | 2 -> 'c' .

@b :: String
decl b = if 1 then 2 else 3

@c :: a -> a
decl c = \x -> x

decl corentin = \a -> "coucou"