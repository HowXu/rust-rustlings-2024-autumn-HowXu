/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/
use std::collections::VecDeque;

// 广度优先搜索

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        //双向图表可访问
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO

        //BFS是要用队列的
        //首先初始化一个返回值
        let mut visit_order = vec![];
        //创建一个表示已经访问的节点的数组
        let mut have_visited = vec![false;self.adj.len()];
        let mut queue = VecDeque::new();//非用不可的队列

        queue.push_back(start); //加入头

        //遍历周围的节点
        while let Some(i) = queue.pop_front() {

            //从队列中访问了这个节点就加进去 这样只有进了队列的才会被push 同时下面有判断机制 队列不会有重复元素
            visit_order.push(i);
            //这里访问了就标记为已访问
            have_visited[i] = true;
            //println!("push {i}");

            for index in self.adj[i].clone() {
                //必须是未访问的才能进入have_visited
                if !have_visited[index] {
                    //标记已经访问
                    have_visited[index] = true;
                    queue.push_back(index); //下一次就从这里开始
                    
                }
            }
        }
        //如此循环直到把队列中所有被push进去的节点访问一遍

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

