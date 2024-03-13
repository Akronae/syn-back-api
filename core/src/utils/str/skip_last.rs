pub trait SkipLast {
    fn skip_last(self, n: usize) -> Self;
}

pub trait SkipLastVec {
    fn skip_last(self, n: usize) -> Self;
}

impl<T> SkipLast for T
where
    T: DoubleEndedIterator,
{
    fn skip_last(mut self, n: usize) -> Self {
        for _ in 0..n {
            self.next_back();
        }
        self
    }
}

impl<T> SkipLastVec for Vec<T> {
    fn skip_last(mut self, n: usize) -> Self {
        self.truncate(self.len().saturating_sub(n));
        self
    }
}
