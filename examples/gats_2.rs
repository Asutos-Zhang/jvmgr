trait LendingIterator {
    type Item<'a> where Self: 'a;


    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}


struct WindowMut<'t,T> {
    slice: &'t mut [T],
    start: usize,
    window_size:usize,
}

impl <'t,T> LendingIterator for WindowMut<'t,T> {
    type Item<'a>  = &'a mut [T] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        let retval = self.slice[self.start..].get_mut(..self.window_size)?;
        self.start +=1;
        Some(retval)
    }
}

fn main(){

}