#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    // Create a new stack
    fn new() -> Self {
        Stack {
            data: Vec::new(),
            size: 0,
        }
    }

    // Push an element onto the stack
    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    // Pop an element from the stack
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

// Function to check if brackets are balanced
fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    
    for c in bracket.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {} // Ignore other characters
        }
    }

    // If the stack is empty, all brackets matched correctly
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bracket_matching_1(){
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_2(){
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_3(){
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_4(){
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_5(){
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_6(){
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
