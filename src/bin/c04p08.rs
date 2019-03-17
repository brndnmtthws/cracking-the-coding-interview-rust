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

    fn common_ancestor_from<F>(
        &self,
        f: &mut F,
        vertex: VertexRef<T>,
        left_vertex: &VertexRef<T>,
        right_vertex: &VertexRef<T>,
    ) -> bool
    where
        F: FnMut(&VertexRef<T>),
    {
        let mut found_one = false;
        for v in vertex.borrow().edges.iter() {
            if Rc::ptr_eq(left_vertex, v) || Rc::ptr_eq(right_vertex, v) {
                if found_one {
                    f(&vertex.clone());
                    return true;
                } else {
                    found_one = true;
                }
            }
        }
        for v in vertex.borrow().edges.iter() {
            if self.common_ancestor_from(f, v.clone(), left_vertex, right_vertex) {
                return true;
            }
        }
        false
    }

    fn common_ancestor<F>(&self, left_vertex: VertexRef<T>, right_vertex: VertexRef<T>, mut f: F)
    where
        F: FnMut(&VertexRef<T>),
    {
        for head in self.head.iter() {
            if self.common_ancestor_from(&mut f, head.clone(), &left_vertex, &right_vertex) {
                return;
            }
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
    fn test_common_ancestor() {
        let mut graph = Graph::<char>::new();
        let c_vertex = graph.add_vertex('c', &[]);
        let d_vertex = graph.add_vertex('d', &[c_vertex.clone()]);
        let a_vertex = graph.add_vertex('a', &[d_vertex.clone()]);
        let b_vertex = graph.add_vertex('b', &[d_vertex.clone()]);
        let _f_vertex = graph.add_vertex('f', &[a_vertex.clone(), b_vertex.clone()]);
        let _e_vertex = graph.add_vertex('e', &[]);

        let mut was_called = false;
        graph.common_ancestor(a_vertex.clone(), b_vertex.clone(), |vertex| {
            was_called = true;
            assert_eq!(Rc::ptr_eq(vertex, &d_vertex), true)
        });
        assert_eq!(was_called, true);
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

    let mut was_called = false;
    graph.common_ancestor(a_vertex.clone(), b_vertex.clone(), |vertex| {
        was_called = true;
        assert_eq!(Rc::ptr_eq(vertex, &d_vertex), true)
    });
    assert_eq!(was_called, true);
}
