use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Type {
    File,
    Directory,
}

#[derive(Debug, Clone)]
struct INode {
    name: String,
    itype: Type,
    owner: String,
    executable: bool,
    writeable: bool,
    children: Vec<Rc<RefCell<INode>>>,
    link_target: Option<Rc<RefCell<INode>>>,
    data: Vec<u8>,
}

impl INode {
    fn add_file(&mut self, name: &str, data: Vec<u8>) -> Rc<RefCell<INode>> {
        let file = Rc::new(RefCell::new(INode {
            name: name.to_string(),
            itype: Type::File,
            owner: "root".to_string(),
            executable: true,
            writeable: true,
            children: vec![],
            link_target: None,
            data,
        }));
        self.children.push(file.clone());
        file.clone()
    }
    fn add_directory(&mut self, name: &str) -> Rc<RefCell<INode>> {
        let dir = Rc::new(RefCell::new(INode {
            name: name.to_string(),
            itype: Type::Directory,
            owner: "root".to_string(),
            executable: true,
            writeable: true,
            children: vec![],
            link_target: None,
            data: vec![],
        }));
        self.children.push(dir.clone());
        dir.clone()
    }
}

#[derive(Debug, Clone)]
struct Filesystem {
    root: INode,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            root: INode {
                name: "".to_string(),
                itype: Type::Directory,
                owner: "root".to_string(),
                executable: true,
                writeable: true,
                children: vec![],
                link_target: None,
                data: vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem() {
        let mut fs = Filesystem::new();
        let derp = fs.root.add_directory("derp");
        let _file = derp.borrow_mut().add_file("herp", vec![0; 5]);
    }
}

fn main() {
    let mut fs = Filesystem::new();
    let derp = fs.root.add_directory("derp");
    let _file = derp.borrow_mut().add_file("herp", vec![0; 5]);
}
