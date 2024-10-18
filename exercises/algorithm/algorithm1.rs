use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Self {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<Box<Node<T>>>,
    end: *mut Node<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: std::ptr::null_mut(),
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut new_node = Box::new(Node::new(obj));
        let node_ptr: *mut _ = &mut *new_node;
        if self.start.is_none() {
            self.start = Some(new_node);
        } else {
            unsafe {
                (*self.end).next = Some(new_node);
            }
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.start;
        for _ in 0..index {
            match current {
                Some(node) => current = &node.next,
                None => return None,
            }
        }
        current.as_ref().map(|node| &node.val)
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord + Clone,
    {
        let mut merged_list = LinkedList::new();

        while list_a.start.is_some() && list_b.start.is_some() {
            let a_value = list_a.start.as_ref().unwrap().val.clone();
            let b_value = list_b.start.as_ref().unwrap().val.clone();

            if a_value <= b_value {
                merged_list.add(a_value);
                list_a.start = list_a.start.unwrap().next;
            } else {
                merged_list.add(b_value);
                list_b.start = list_b.start.unwrap().next;
            }
        }

        // Append the rest of list_a if any
        while let Some(node) = list_a.start {
            merged_list.add(node.val.clone());
            list_a.start = node.next;
        }

        // Append the rest of list_b if any
        while let Some(node) = list_b.start {
            merged_list.add(node.val.clone());
            list_b.start = node.next;
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut current = &self.start;
        while let Some(node) = current {
            write!(f, "{} ", node.val)?;
            current = &node.next;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        let list_c = LinkedList::<i32>::merge(list_a, list_b);

        for (i, &expected_value) in target_vec.iter().enumerate() {
            assert_eq!(*list_c.get(i).unwrap(), expected_value);
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        let list_c = LinkedList::<i32>::merge(list_a, list_b);

        for (i, &expected_value) in target_vec.iter().enumerate() {
            assert_eq!(*list_c.get(i).unwrap(), expected_value);
        }
    }
}
