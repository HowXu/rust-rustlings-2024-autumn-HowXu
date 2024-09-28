/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

//这里使用Quicksort快速排序
//获得两端点的索引

//use std::fmt::*;  + Display + Debug
fn quickSort<T: PartialOrd + Clone>(array: &mut [T], left: usize, right: usize) {
    let mut index_left = left;
    let mut index_right = right;

    //基准数在左 cache机制
    let i = array[left].clone();

    while index_left < index_right {
        //同时移动位置

        //寻找右边比基准数小的数的索引

        //因为循环是有先后的，所以要加上left < right的条件
        while index_left < index_right && array[index_right] >= i {
            index_right -= 1;
            //println!(" index_right {index_right}");
            //println!(" array[index_right] {}", array[index_right]);
        }
        //退出循环这里说明找到了 这个数要和上一个left交换
        array.swap(index_right, index_left);

        //寻找左边比基准数大的数的索引
        while index_left < index_right && array[index_left] <= i {
            index_left += 1;
            //println!(" index_left {index_left}");
            //println!(" array[index_left] {}", array[index_left]);
        }

        //这个数要和right交换 因为众所周知的rust借用问题 这里用swap方法
        array.swap(index_right, index_left);
        //println!("{:?}", array);
    }

    //左等于右说明是单数据了 这里return结束递归

    if left == right {
        return;
    }

    //接下来分别处理left左右两边
    //这里直接用的left作为索引值 因此在下一个递归的地方要判断index_left防止溢出
    quickSort(array, left, index_left);
    if index_left != array.len() - 1 {
        quickSort(array, index_left + 1, right);
    }
}

//要加这个不然泛型不能比大小

// + Display + Debug
fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    //TODO
    if array.len() > 1 {
        quickSort(array, 0, array.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
