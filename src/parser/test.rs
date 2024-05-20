use crate::ast::*;



#[cfg(test)]
fn check_ast(content: &str, expected: Program<Span>) {
    let mut parser = crate::parser::Parser::new(content);
    let ast = parser.parse();
    assert_eq!(ast, Ok(expected));
}

macro_rules! span {
    ($s:expr, $e:expr, $i:expr) => {
        Span {
            start: $s,
            end: $e,
            input: $i.to_string(),
        }
    };
}


#[test]
fn test_empty_program() {
    check_ast("", Program { statements: vec![] });
}



#[test]
fn test_bind() {
    check_ast(
        "x = 5",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("x".to_string(), span!(0, 1, "x")), 
                    vec![], 
                    ParsedExpr::Literal(Literal::new(
                        LiteralKind::Integer(5),
                        span!(4, 5, "5")
                    )), 
                    span!(0, 5, "x = 5")
                )
            )],
        },
    );
}

#[test]
fn test_bind_with_params() {
    check_ast(
        "f x y = 3",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("f".to_string(), span!(0, 1, "f")), 
                    vec![
                        Pattern::Variable(
                            Identifier::new("x".to_string(), span!(2, 3, "x"))
                        ),
                        Pattern::Variable(
                            Identifier::new("y".to_string(), span!(4, 5, "y"))
                        ),
                    ], 
                    ParsedExpr::Literal(
                        Literal::new(
                            LiteralKind::Integer(3),
                            span!(8, 9, "3")
                        )
                    
                    ), 
                    span!(0, 9, "f x y = 3")
                )
            )],
        },
    );
}

#[test]
fn test_literal() {
    check_ast(
        "a = 3",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("a".to_string(), span!(0, 1, "a")), 
                    vec![], 
                    ParsedExpr::Literal(
                        Literal::new(
                            LiteralKind::Integer(3),
                            span!(4, 5, "3")
                        )
                    ), 
                    span!(0, 5, "a = 3")
                )
            )],
        },
    );

    check_ast("a = \"hello\"", Program {
        statements: vec![Statement::Bind(
            Bind::new(
                Identifier::new("a".to_string(), span!(0, 1, "a")), 
                vec![], 
                ParsedExpr::Literal(
                    Literal::new(
                        LiteralKind::String("hello".to_string()),
                        span!(4, 11, "\"hello\"")
                    )
                ), 
                span!(0, 11, "a = \"hello\"")
            )
        )],
    });

    check_ast("a = True", Program {
        statements: vec![Statement::Bind(
            Bind::new(
                Identifier::new("a".to_string(), span!(0, 1, "a")), 
                vec![], 
                ParsedExpr::Literal(
                    Literal::new(
                        LiteralKind::Bool(Bool::True),
                        span!(4, 8, "True")
                    )
                ), 
                span!(0, 8, "a = True")
            )
        )],
    });

    check_ast("a = False", Program {
        statements: vec![Statement::Bind(
            Bind::new(
                Identifier::new("a".to_string(), span!(0, 1, "a")), 
                vec![], 
                ParsedExpr::Literal(
                    Literal::new(
                        LiteralKind::Bool(Bool::False),
                        span!(4, 9, "False")
                    )
                ), 
                span!(0, 9, "a = False")
            )
        )],
    });

    check_ast("a = 3.14", Program {
        statements: vec![Statement::Bind(
            Bind::new(
                Identifier::new("a".to_string(), span!(0, 1, "a")), 
                vec![], 
                ParsedExpr::Literal(
                    Literal::new(
                        LiteralKind::Float(3.14),
                        span!(4, 8, "3.14")
                    )
                ), 
                span!(0, 8, "a = 3.14")
            )
        )],
    });

}

#[test]
fn test_app() {
    check_ast(
        "a = f 3",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("a".to_string(), span!(0, 1, "a")), 
                    vec![], 
                    ParsedExpr::App(
                        App::new(
                            Identifier::new("f".to_string(), span!(4, 5, "f")), 
                            vec![
                                ParsedExpr::Literal(
                                    Literal::new(
                                        LiteralKind::Integer(3),
                                        span!(6, 7, "3")
                                    )
                                )
                            ], 
                            span!(4, 7, "f 3")
                        )
                    ), 
                    span!(0, 7, "a = f 3")
                )
            )],
        },
    );

}

#[test]
fn test_list() {
    check_ast(
        "a = [1, 2, 3]",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("a".to_string(), span!(0, 1, "a")), 
                    vec![], 
                    AnnExpr::List { list: 
                            vec![
                                AnnExpr::Literal(
                                    Literal::new(
                                        LiteralKind::Integer(1),
                                        span!(5, 6, "1")
                                    )
                                ),
                                AnnExpr::Literal(
                                    Literal::new(
                                        LiteralKind::Integer(2),
                                        span!(8, 9, "2")
                                    )
                                ),
                                AnnExpr::Literal(
                                    Literal::new(
                                        LiteralKind::Integer(3),
                                        span!(11, 12, "3")
                                    )
                                ),
                            ], 
                    ann: span!(4, 13, "[1, 2, 3]")
                }, 
                    span!(0, 13, "a = [1, 2, 3]")
                )
            )],
        },
    );

}

#[test]
fn test_let() {
    check_ast(
        "a = let x = 3; in x",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("a".to_string(), span!(0, 1, "a")), 
                    vec![], 
                    AnnExpr::Let(
                        vec![Bind::new(
                            Identifier::new("x".to_string(), span!(8, 9, "x")), 
                            vec![], 
                            ParsedExpr::Literal(
                                Literal::new(
                                    LiteralKind::Integer(3),
                                    span!(12, 13, "3")
                                )
                            ), 
                            span!(8, 13, "x = 3")
                        )], 
                        Box::new(AnnExpr::Identifier(
                            Identifier::new("x".to_string(), span!(18, 19, "x"))
                        )), 
                        span!(4, 19, "let x = 3; in x")
                    ), 
                    span!(0, 19, "a = let x = 3; in x")
                )
            )],
        },
    );
}

#[test]
fn test_match() {
    check_ast(
        "a = match 3 with | 3 -> 4 | 4 -> 5",
        Program {
            statements: vec![Statement::Bind(
                Bind::new(
                    Identifier::new("a".to_string(), span!(0, 1, "a")), 
                    vec![], 
                    ParsedExpr::Match(
                        Box::new(ParsedExpr::Literal(
                            Literal::new(
                                LiteralKind::Integer(3),
                                span!(10, 11, "3")
                            )
                        )), 
                        vec![
                            (
                                Box::new(
                                    Pattern::Literal(
                                        Literal::new(
                                            LiteralKind::Integer(3),
                                            span!(19, 20, "3")
                                        )
                                    )
                                ),
                                Box::new(
                                    ParsedExpr::Literal(
                                        Literal::new(
                                            LiteralKind::Integer(4),
                                            span!(24, 25, "4")
                                        )
                                    )
                                )
                            ),
                            (
                                Box::new(
                                    Pattern::Literal(
                                        Literal::new(
                                            LiteralKind::Integer(4),
                                            span!(28, 29, "4")
                                        )
                                    )
                                ),
                                Box::new(
                                    ParsedExpr::Literal(
                                        Literal::new(
                                            LiteralKind::Integer(5),
                                            span!(33, 34, "5")
                                        )
                                    )
                                )
                            ),

                        ], 
                        span!(4, 34, "match 3 with | 3 -> 4 | 4 -> 5")
                    ), 
                    span!(0, 34, "a = match 3 with | 3 -> 4 | 4 -> 5")
                )
            )],
        },
    );
}