/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

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
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let n = self.adj.len();
        if start >= n {
            // 如果起始节点超出范围，返回空向量
            return vec![];
        }

        let mut visited = vec![false; n]; // 记录每个节点是否被访问过
        let mut queue = VecDeque::new();  // BFS 使用的队列
        let mut visit_order = Vec::new(); // 记录访问顺序

        // 初始化：将起始节点加入队列并标记为已访问
        queue.push_back(start);
        visited[start] = true;

        // 开始 BFS
        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 记录当前访问的节点

            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;          // 标记为已访问
                    queue.push_back(neighbor);        // 加入队列
                }
            }
        }

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

    #[test]
    fn test_bfs_invalid_start() {
        let graph = Graph::new(3);
        let visited_order = graph.bfs_with_return(5);
        assert_eq!(visited_order, vec![]);
    }
}