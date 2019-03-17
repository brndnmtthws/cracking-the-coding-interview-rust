use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type VertexRef<T> = Rc<RefCell<Vertex<T>>>;

struct Graph<T> {
    head: Option<VertexRef<T>>,
}

struct Vertex<T> {
    data: T,
    edges: Vec<VertexRef<T>>,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn add_vertex(&mut self, value: T, parent_vertices: &[VertexRef<T>]) -> VertexRef<T> {
        let ret = Rc::new(RefCell::new(Vertex {
            data: value,
            edges: Vec::<VertexRef<T>>::new(),
        }));
        if parent_vertices.is_empty() {
            self.head = Some(ret.clone());
        } else {
            for parent in parent_vertices {
                parent.borrow_mut().edges.push(ret.clone());
            }
        }
        ret
    }

    fn dfs_from<F>(&self, f: &mut F, vertex: VertexRef<T>) -> bool
    where
        F: FnMut(&VertexRef<T>) -> bool,
    {
        for v in vertex.borrow().edges.iter() {
            if !self.dfs_from(f, v.clone()) {
                return false;
            }
        }
        f(&vertex.clone())
    }

    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(&VertexRef<T>) -> bool,
    {
        if let Some(head) = &self.head {
            self.dfs_from(&mut f, head.clone());
        }
    }

    fn has_path(&self, from: VertexRef<T>, to: VertexRef<T>) -> bool {
        let mut found_path = false;
        self.dfs_from(
            &mut |v| {
                if Rc::ptr_eq(v, &to) {
                    // a path has been found
                    found_path = true;
                    false
                } else {
                    true
                }
            },
            from.clone(),
        );
        found_path
    }
}

impl<T: Display> Display for Graph<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        self.dfs(|v| {
            write!(w, "{}, ", v.borrow().data).unwrap();
            true
        });
        write!(w, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_between_nodes() {
        let mut graph = Graph::<i32>::new();
        let first = graph.add_vertex(1, &[]);
        let second = graph.add_vertex(2, &[first.clone()]);
        let third = graph.add_vertex(3, &[first.clone(), second.clone()]);

        assert_eq!(graph.has_path(first.clone(), third.clone()), true);
        assert_eq!(graph.has_path(third.clone(), first.clone()), false);
    }
}

fn main() {
    let mut graph = Graph::<i32>::new();
    let first = graph.add_vertex(1, &[]);
    let second = graph.add_vertex(2, &[]);
    graph.has_path(first.clone(), second.clone());
}
