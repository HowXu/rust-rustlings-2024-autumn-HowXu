/*
    graph
    This problem requires you to implement a basic graph function
*/

// 图

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    //写无向图的节点连接的函数
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        //因为是无向图 所以要接过去接过来

        let (node1, node2, val) = edge; //拆包

        // 先把两个节点挂上去
        self.add_node(node1);
        self.add_node(node2);
        //加entry防止重复造节点
        self.adjacency_table_mutable()
            .entry(node1.into())
            .or_insert(vec![])
            .push((node2.into(), val)); //如果该节点已经存在，则直接返回该值的可变引用 不存在则创建一个空向量再传入指向信息
        self.adjacency_table_mutable()
            .entry(node2.into())
            .or_insert(vec![])
            .push((node1.into(), val));
        //insert是按照键值对加向量 返回值是插入向量的引用
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //为表添加节点
        //首先检测这个节点值是否存在 存在就false
        if self.adjacency_table().contains_key(node.into()) {
            return false;
        } else {
            //不存在时在adjacency_table_mutable添加这个节点并给一个没有标注连接的向量
            self.adjacency_table_mutable().insert(node.into(), vec![]);
            return true;
        }
        //TODO
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        //这个是有向图吗
        //这个也不用写吗
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
