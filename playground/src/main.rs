
struct user{
    name : String,
    age : u8,
    description : String
}

#[derive(Debug)]
enum Type {
    Nucleo,
    Discovery
}

#[derive(Debug)]
enum Board
{
    Name(String),
    Type(Type),
    Version(u8,u8)
}

enum Option<Board>
{
    Some(Board),
    None
}

fn ops_func(ops: Option<Board>) {
    match ops {
        Option::Some(b) => println!("te be to {:?}", b),
        Option::None => println!("pusto")
    }
}
fn main()
{
    let _f756 = Board::Version(2, 3);
    let _xx = Board::Type(Type::Nucleo);
    let _xxx = Board::Type(Type::Discovery);
    match _xxx {
        Board::Name(s) => println!("found name: {}", s),
        Board::Version(2,3) => println!("found a Version"),
        Board::Type(t) => println!("It's a {:?}", t),
        _ => println!("nevermind")
    }

    let some_board = Option::Some(Board::Type(Type::Nucleo));
    let absent_board: Option<Board> = Option::None;

    ops_func(some_board);
    ops_func(absent_board);

    
}
