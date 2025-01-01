use std::{thread, time::Duration};

static CLEAR: &str = "\x1B[2J\x1B[H";

struct Unbounded;
struct Bounded {
    size: usize,
    delims: (char, char),
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.size - progress.i),
            self.delims.1
        );
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Bounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            size: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_op() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let delims = ('{', '}');
    let v = [1, 2, 3, 4];
    // for _ in (0..).progress() {
    //     expensive_op();
    // }

    // v.iter().progress().with_delims(delims).with_bound()
    for _ in v.iter().progress().with_bound().with_delims(delims) {
        expensive_op();
    }
}
