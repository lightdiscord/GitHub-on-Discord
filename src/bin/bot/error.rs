error_chain! {
    foreign_links {
        Var(::std::env::VarError);
        Serenity(::serenity::Error);
    }
}
