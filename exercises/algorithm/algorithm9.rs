/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,          // 堆中有效元素的数量
    items: Vec<T>,         // 存储堆元素的向量（索引从1开始，方便计算父子节点）
    comparator: fn(&T, &T) -> bool,  // 比较器：最小堆用 `a < b`，最大堆用 `a > b`
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],  // 索引0占位，实际元素从索引1开始
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加元素（上浮调整：维护堆性质）
    pub fn add(&mut self, value: T) {
        // 1. 将新元素添加到向量末尾（堆的最底层叶子节点）
        self.items.push(value);
        self.count += 1;

        // 2. 上浮调整：从当前节点（新元素）向上对比父节点，不符合堆性质则交换
        let mut current_idx = self.count;  // 新元素的初始索引（从1开始）
        while current_idx > 1 {  // 根节点（索引1）没有父节点，无需继续
            let parent_idx = self.parent_idx(current_idx);
            
            // 若当前节点符合堆性质（通过比较器判断），则调整结束
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                // 交换当前节点与父节点
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;  // 继续向上检查父节点
            } else {
                break;  // 堆性质已满足，退出循环
            }
        }
    }

    /// 获取父节点索引（索引从1开始）
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 判断当前节点是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 获取左子节点索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 获取右子节点索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 找到当前节点的“最优子节点”（最小堆找最小子节点，最大堆找最大子节点）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 1. 若只有左子节点，直接返回左子节点索引
        if right_idx > self.count {
            return left_idx;
        }

        // 2. 比较左右子节点，返回符合堆性质的子节点索引
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx  // 左子节点更优（更小/更大）
        } else {
            right_idx  // 右子节点更优
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆（比较器：a < b）
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆（比较器：a > b）
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

/// 实现迭代器：每次提取堆顶元素（堆的核心功能）
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 1. 堆为空时返回None
        if self.is_empty() {
            return None;
        }

        // 2. 提取堆顶元素（索引1，堆的根节点）
        let top = self.items.swap_remove(1);  // 交换并删除索引1的元素，末尾元素补位
        self.count -= 1;

        // 3. 下沉调整：从新的根节点（原末尾元素）向下对比子节点，维护堆性质
        let mut current_idx = 1;
        while self.children_present(current_idx) {  // 有子节点时才需要调整
            let best_child_idx = self.smallest_child_idx(current_idx);
            
            // 若当前节点不符合堆性质，交换当前节点与最优子节点
            if (self.comparator)(&self.items[best_child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, best_child_idx);
                current_idx = best_child_idx;  // 继续向下检查子节点
            } else {
                break;  // 堆性质已满足，退出循环
            }
        }

        Some(top)  // 返回提取的堆顶元素
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)  // 最小堆比较器
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)  // 最大堆比较器
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
        assert_eq!(heap.next(), Some(2));  // 堆顶最小元素
        assert_eq!(heap.next(), Some(4));  // 下一个最小元素
        assert_eq!(heap.next(), Some(9));  // 下一个最小元素
        heap.add(1);  // 插入更小元素，会上浮到堆顶
        assert_eq!(heap.next(), Some(1));  // 提取新的最小元素
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));  // 堆顶最大元素
        assert_eq!(heap.next(), Some(9));   // 下一个最大元素
        assert_eq!(heap.next(), Some(4));   // 下一个最大元素
        heap.add(1);  // 插入更小元素，会下沉到叶子
        assert_eq!(heap.next(), Some(2));   // 提取剩余最大元素
    }
}