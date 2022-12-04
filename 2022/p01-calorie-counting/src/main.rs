use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Top N chunks to add.
    n: usize,
    /// File with the data to read.
    file_name: String,
}

fn main() {
    let Args { n, file_name } = Args::parse();
    let data = std::fs::read_to_string(file_name).expect("File must exist");
    let mut chunks: Vec<_> = ChunkSumsIter {
        inner: data.lines().map(|l| l.parse::<usize>().ok()),
    }
    .collect();
    chunks.sort();
    let max: usize = chunks.into_iter().rev().take(n).sum();
    println!("max: {max}")
}

struct ChunkSumsIter<T: Iterator<Item = Option<usize>>> {
    inner: T,
}

impl<T: Iterator<Item = Option<usize>>> Iterator for ChunkSumsIter<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = None;
        while let Some(Some(i)) = self.inner.next() {
            item = match item {
                Some(acc) => Some(acc + i),
                None => Some(i),
            }
        }
        item
    }
}
