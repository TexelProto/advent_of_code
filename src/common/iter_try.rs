pub fn try_flatten<T,E,TS,I>(iter: I) -> TryFlatten<T,E,TS,I>
where 
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>{
    TryFlatten{super_iter: iter, current_iter: None}
}

pub struct TryFlatten<T,E,TS,I>
where 
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>{
    super_iter: I,
    current_iter: Option<ResultIter<TS::IntoIter,E>>,
}

enum ResultIter<I: Iterator, E> {
    Ok(I),
    Err(std::option::IntoIter<E>),
}

impl<T,E,TS,I> Iterator for TryFlatten<T,E,TS,I>
where 
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>{

    type Item = Result<T,E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut iter = match &mut self.current_iter {
                // a subiterator is present try to continue with it
                Some(iter) => iter,
                // subiterator is missing, try creating a new one
                None => match self.super_iter.next() {
                    // there was at least A result that should be yielded
                    Some(res) => {
                        let iter: ResultIter<TS::IntoIter,E> = match res {
                            Ok(ok) => ResultIter::Ok(ok.into_iter()),
                            Err(err) => ResultIter::Err(Some(err).into_iter()),
                        };
                        self.current_iter = Some(iter);
                        self.current_iter.as_mut().unwrap()
                    },
                    // sub- and super-iterator are exhausted: were done 
                    None => return None,
                }
            };

            // try continuing with the current sub iterator
            match &mut iter {
                ResultIter::Ok(ok) => match ok.next() {
                    Some(x) => return Some(Ok(x)),
                    None => continue,
                }
                ResultIter::Err(err) => match err.next() {
                    Some(x) => return Some(Err(x)),
                    None => continue,
                }
            }
        }
    }
}