pub enum TypeCheckerErrorKind {
    MismatchedTypes,
    NonExhaustiveMatch,
    InfiniteType,
    AmbiguousType,
    IncompatibleTypes,
    Unification,
}
