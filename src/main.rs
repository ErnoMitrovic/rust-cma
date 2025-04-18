fn main() {
    let source = std::fs::read_to_string("example.c").expect("Could not read file");
    let tokens = lexer::tokenize(&source);
    println!("Tokens: {:?}", tokens);

    let ast = parser::parse(&source).expect("Parsing failed");
    println!("AST: {:?}", ast);

    semantic::analyze(&ast).expect("Semantic analysis failed");
    codegen::generate(&ast);
}