fn main() {
    println!("Hello, world!");
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            value,
            next: std::ptr::null_mut(),
        }))
    }
}

struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            let node = Node::new(value);

            if self.head.is_null() {
                self.head = node;
            } else {
                (*node).next = self.head;
                self.head = node;
            }

            self.tail = node
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                // from_raw turns the ptr back to smart ptr and makes sure to clean up at the end of the scope.
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if head.next.is_null() {
                    self.tail = std::ptr::null_mut();
                }

                Some(head.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|n| &n.value) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|n| &mut n.value) }
    }

    pub fn iter(&self) -> ListIter<'_, T> {
        unsafe {
            ListIter {
                next: Some(&(*self.head)),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.next {
            unsafe {
                if n.next.is_null() {
                    self.next = None
                } else {
                    self.next = Some(&(*n.next));
                }
            }

            Some(&n.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_list() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));

        let popped = list.pop();

        assert_eq!(popped, Some(3));
        assert_eq!(list.peek(), Some(&2));

        if let Some(v) = list.peek_mut() {
            *v = 10;
        };

        assert_eq!(list.peek(), Some(&10))
    }

    #[test]
    fn test_list_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut list_iter = list.iter();

        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
    }
}
