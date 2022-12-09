#[derive(Debug)]
pub enum Command<'a> {
    Cd(&'a str),
    Ls,
}

#[derive(Debug)]
pub enum Response<'a> {
    File { name: &'a str, size: usize },
    Dir { name: &'a str },
}

#[derive(Debug)]
pub enum WalkItem<'a> {
    Command(Command<'a>),
    Response(Response<'a>),
}
