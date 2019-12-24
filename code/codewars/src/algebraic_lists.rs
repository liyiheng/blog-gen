impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut elements: Vec<Cons<T>> = it.into_iter().map(|t| Cons::new(t, Cons::Null)).collect();
        while elements.len() > 1 {
            let tail = elements.pop().unwrap();
            if let Cons::Cons(_, next) = elements.last_mut().unwrap() {
                *next = Box::new(tail);
            }
        }
        if elements.is_empty() {
            return Cons::Null;
        }
        elements.pop().unwrap()
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut elements = vec![];
        let mut head = self;
        loop {
            if let Cons::Cons(t, next) = head {
                elements.push(t.clone());
                head = next;
            } else {
                break;
            }
        }
        elements
    }

    pub fn filter<F>(&self, fun: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let elements: Vec<T> = self.to_vec().into_iter().filter(&fun).collect();
        Self::from_iter(elements)
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
    where
        F: Fn(T) -> S,
        S: Clone,
    {
        let elements: Vec<S> = self.to_vec().into_iter().map(&fun).collect();
        Cons::from_iter::<Vec<S>>(elements)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }
}

//#[test]
fn should_create_from_vec() {
    assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

    assert_eq!(
        Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
        vec![1, 2, 3, 4, 5]
    );
}

//  #[test]
fn should_filter() {
    assert_eq!(
        Cons::from_iter(vec![1, 2, 3, 4, 5])
            .filter(|&n| n > 3)
            .to_vec(),
        vec![4, 5]
    );

    assert_eq!(
        Cons::from_iter(vec![1, 2, 3, 4, 5]).filter(|&n| n > 5),
        Cons::Null
    );
}

//   #[test]
fn should_map() {
    assert_eq!(
        Cons::from_iter(vec!["1", "2", "3", "4", "5"])
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .to_vec(),
        vec![1, 2, 3, 4, 5]
    );
}

//    #[test]
fn should_filter_map() {
    assert_eq!(
        Cons::from_iter(vec![1, 2, 3, 4, 5])
            .filter(|n| n % 2 == 0)
            .map(|x| x.to_string())
            .to_vec(),
        ["2", "4"]
    );
}
