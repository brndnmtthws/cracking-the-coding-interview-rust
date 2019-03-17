use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type VertexRef<T> = Rc<RefCell<Vertex<T>>>;

struct Graph<T> {
    head: Vec<VertexRef<T>>,
}

struct Vertex<T> {
    data: T,
    edges: Vec<VertexRef<T>>,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self {
            head: Vec::<VertexRef<T>>::new(),
        }
    }

    fn add_vertex(&mut self, value: T, parent_vertices: &[VertexRef<T>]) -> VertexRef<T> {
        let ret = Rc::new(RefCell::new(Vertex {
            data: value,
            edges: Vec::<VertexRef<T>>::new(),
        }));
        if parent_vertices.is_empty() {
            self.head.push(ret.clone());
        } else {
            for parent in parent_vertices {
                parent.borrow_mut().edges.push(ret.clone());
            }
        }
        ret
    }

    fn dfs_from<F>(&self, f: &mut F, vertex: VertexRef<T>)
    where
        F: FnMut(&VertexRef<T>),
    {
        for v in vertex.borrow().edges.iter() {
            self.dfs_from(f, v.clone());
        }
        f(&vertex.clone());
    }

    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(&VertexRef<T>),
    {
        for head in self.head.iter() {
            self.dfs_from(&mut f, head.clone());
        }
    }
}

impl<T: Display> Display for Graph<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        self.dfs(|v| {
            write!(w, "{}, ", v.borrow().data).unwrap();
        });
        write!(w, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependencies() {
        let mut graph = Graph::<char>::new();
        let c_vertex = graph.add_vertex('c', &[]);
        let d_vertex = graph.add_vertex('d', &[c_vertex.clone()]);
        let a_vertex = graph.add_vertex('a', &[d_vertex.clone()]);
        let b_vertex = graph.add_vertex('b', &[d_vertex.clone()]);
        let _f_vertex = graph.add_vertex('f', &[a_vertex.clone(), b_vertex.clone()]);
        let _e_vertex = graph.add_vertex('e', &[]);

        let mut graph_order = Vec::<char>::new();
        graph.dfs(|vertex| {
            let value = vertex.borrow().data;
            if !graph_order.contains(&value) {
                graph_order.push(value);
            }
        });
        assert_eq!(graph_order, vec!['f', 'a', 'b', 'd', 'c', 'e']);
    }
}

fn main() {
    let mut graph = Graph::<char>::new();
    let c_vertex = graph.add_vertex('c', &[]);
    let d_vertex = graph.add_vertex('d', &[c_vertex.clone()]);
    let a_vertex = graph.add_vertex('a', &[d_vertex.clone()]);
    let b_vertex = graph.add_vertex('b', &[d_vertex.clone()]);
    let _f_vertex = graph.add_vertex('f', &[a_vertex.clone(), b_vertex.clone()]);
    let _e_vertex = graph.add_vertex('e', &[]);

    let mut graph_order = Vec::<char>::new();
    graph.dfs(|vertex| {
        let value = vertex.borrow().data;
        if !graph_order.contains(&value) {
            graph_order.push(value);
        }
    });
    assert_eq!(graph_order, vec!['f', 'a', 'b', 'd', 'c', 'e']);
}
