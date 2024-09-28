/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/

//队列

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    //TODO
    //用queue实现栈堆
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 让q1作为主用 q2作为辅助 q2接到数据然后把q1的接上去 然后再把q2转到q1里面
        // 这个意义上讲q2只在这里一个地方用啊，完全可以使用暂时的变量
        self.q2.enqueue(elem);
        while !self.q1.is_empty() {
            if let Ok(get) = self.q1.dequeue() {
                self.q2.enqueue(get);
            }
        }
        //这样就把q1转到q2了 然后再轮回去
        while !self.q2.is_empty() {
            if let Ok(get) = self.q2.dequeue() {
                self.q1.enqueue(get);
            }
        }

        //最后这个while可以换成 std::mem::swap(&mut self.q1, &mut self.q2); 交换两个queue的地址 
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        self.q1.dequeue()
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
