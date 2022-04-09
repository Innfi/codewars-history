//https://www.codewars.com/kata/529a92d9aba78c356b000353

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    enum Cons<T: Clone> {
        Cons(T, Box<Cons<T>>),
        Null
    }

    impl<T: Clone> Cons<T> {
        pub fn new(head: T, tail: Self) -> Self {
            Cons::Cons(head, Box::new(tail))
        }

        pub fn to_vec(&self) -> Vec<T> {
            match self {
                &Cons::Null => vec![],
                &Cons::Cons(ref head, ref tail) => {
                    let mut head = vec![head.clone()];
                    head.extend(tail.to_vec());
                    head
                }
            }
        }
    }

    impl<T: Clone> Cons<T> {
        pub fn from_iter<I>(it: I) -> Self
            where I: IntoIterator<Item=T> {
            Cons::gen(it.into_iter())
        }

        pub fn gen<I>(mut iter: I) -> Self
            where I: Iterator<Item=T> {
            let element = iter.next();
            if !element.is_some() { return Cons::Null; }

            Cons::new(element.unwrap(), Cons::gen(iter))
        }

        pub fn filter<F>(&self, fun: F) -> Self
            where F: Fn(&T) -> bool {
            let out_vec: Vec<T> = self.to_vec()
                .iter()
                .filter(|x| fun(x))
                .cloned()
                .collect();

            if out_vec.is_empty() { return Cons::Null; }

            Cons::from_iter(out_vec)
        }

        pub fn map<F,S>(&self, fun: F) -> Cons<S>
            where F: Fn(T) -> S, S: Clone {
            let out_vec: Vec<S> = self.to_vec()
                .iter()
                .map(|x| fun(x.clone()))
                .collect();
            
            if out_vec.is_empty() { return Cons::Null; }
            
            Cons::from_iter(out_vec)
        }
    }

    #[test]
    fn should_create_from_vec() {
        assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);
        
        assert_eq!(Cons::from_iter(vec![1,2,3,4,5]).to_vec(),
             vec![1,2,3,4,5]);
    }

    #[test]
    fn should_filter() {
    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
        .filter(|&n| n > 3)
        .to_vec(),
        vec![4,5]);
                    
    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
        .filter(|&n| n > 5),
        Cons::Null);
    }

    #[test]
    fn should_map() {
        assert_eq!(Cons::from_iter(vec!["1","2","3","4","5"])
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .to_vec(),
        vec![1,2,3,4,5]);
    }


    #[test]
    fn should_filter_map() {
    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
        .filter(|n| n % 2 == 0)
        .map(|x| x.to_string())
        .to_vec(),
    ["2","4"]);
    }
}
