/*
    heap
    This question requires you to implement a binary heap function
*/

// 堆

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], //这里第一个值是存在的
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    //这里的add要满足shift_add的要求
    pub fn add(&mut self, value: T) {
        //TODO
        //首先插进去
        self.items.push(value);
        self.count += 1;
        //这里的shift_up应该是不断让数字向上浮动
        //self.shift_up(self.count);
        //访问父节点 比较二者大小 如果后者大就应该交换 即继续shift_up

        let mut index = self.count;

        let mut fatr_index = self.parent_idx(index);
        // while value > self.items[fatr_index] 考虑是两种类型的堆一起写 不能这样比大小 应该用实现方法 泛型你怎么能这么恶心
        // 这里father_index为0就说明到头了 father为0说明到了根节点
        while fatr_index != 0 && (self.comparator)(&self.items[index], &self.items[fatr_index]) {
            //获取父节点索引 应该要不断更新
            //比较通过了就换位置
            self.items.swap(index, fatr_index);
            //然后继续向上比较 继续换位置
            index = fatr_index;

            fatr_index = self.parent_idx(index);
        }
    }

    //fn shift_up(&mut self, mut index: usize) {}

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    //判断这个节点是不是没有后续的子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        //返回idx的最Max/min的children的索引

        //先拿
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        //堆是有可能没有right_child的 这个时候应该返回left_child

        if right_idx > self.count {
            return left_idx;
        }

        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            return left_idx;
        } else {
            return right_idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default
{
    type Item = T;

    //实现堆的迭代器?
    //堆中每次都只能删除堆顶元素。为了便于重建堆，实际的操作是将最后一个数据的值赋给根结点，然后再从根结点开始进行一次从上向下的调整。
    // 调整时先在左右子结点中找最小的，如果父结点比这个最小的子结点还小说明不需要调整了，反之将父结点和它交换后再考虑后面的结点。
    //相当于根结点数据的“下沉”过程。
    fn next(&mut self) -> Option<T> {

        //当要存储有 N 个节点的二叉堆时，一般会用长度为 N+1 的数组来存储这颗二叉堆。数组索引为 1 元素就是二叉堆的根节点。
        //索引 0 的位置就空着不用，这是为了方便计算每颗子树的子节点和父节点的索引。
        //TODO
        //这个next应该返回第一个左节点
        //None
        //有点pop的意思，这个值必须删掉，然后堆的结构依然保持完整
        if self.is_empty(){
            return None;
        }
        //这里涉及一个堆的向上提升操作
        //首先把根节点和最后交换
        self.items.swap(1, self.count); //因为clone的限制只能这样做
        //self.items.insert(self.count,self.items[1].clone());

        self.count -= 1; //获得最后节点的上一个节点
        //这里就是拿到的next值
        let index_1_content = self.items.pop().unwrap(); //弹出root节点的值
        //接下来提升整个堆 从新的根节点开始
        let mut idx = 1; 
        //获取根节点子节点中最小的
        let mut smallest_child_idx = self.smallest_child_idx(idx);

        //如果是没有后续的子节点 就可以结束了
        while self.children_present(idx)  {
            //如果当前节点在与另一个节点(直接获取最小值)比较中 max比子节点最大的大 比min中最小的小 就结束了
            if  (self.comparator)(&self.items[idx], &self.items[smallest_child_idx]) {
                break;
            }
            //如果不是 就交换二者位置 继续进行替换
            self.items.swap(idx, smallest_child_idx);
            idx = smallest_child_idx;
            smallest_child_idx = self.smallest_child_idx(idx);
        }

        Some(index_1_content)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
