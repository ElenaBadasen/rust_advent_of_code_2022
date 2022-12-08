use std::cell::RefCell;
use std::io;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum FileType {
    Dir,
    File,
}

trait Sizable {
    fn size(&self) -> u32;
    fn parent(&self) -> Option<Rc<Box<dyn Sizable>>>;
    fn file_type(&self) -> FileType;
    fn name(&self) -> &String;
    fn children(&self) -> Option<&RefCell<Vec<Rc<Box<dyn Sizable>>>>>;
}

struct Dir {
    name: String,
    parent_dir: Option<Weak<Box<dyn Sizable>>>,
    children: RefCell<Vec<Rc<Box<dyn Sizable>>>>,
}

impl Dir {
    fn new(
        name: String,
        parent_dir: Option<Weak<Box<dyn Sizable>>>,
        children: Vec<Rc<Box<dyn Sizable>>>,
    ) -> Dir {
        Dir {
            name,
            parent_dir,
            children: RefCell::new(children),
        }
    }
}

impl Sizable for Dir {
    fn size(&self) -> u32 {
        self.children
            .borrow()
            .iter()
            .map(|child| child.size())
            .sum()
    }

    fn parent(&self) -> Option<Rc<Box<dyn Sizable>>> {
        self.parent_dir.as_ref()?.upgrade()
    }

    fn file_type(&self) -> FileType {
        FileType::Dir
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn children(&self) -> Option<&RefCell<Vec<Rc<Box<dyn Sizable>>>>> {
        Some(&self.children)
    }
}

struct File {
    size: u32,
    name: String,
    parent_dir: Weak<Box<dyn Sizable>>,
}

impl Sizable for File {
    fn size(&self) -> u32 {
        self.size
    }

    fn parent(&self) -> Option<Rc<Box<dyn Sizable>>> {
        self.parent_dir.upgrade()
    }

    fn file_type(&self) -> FileType {
        FileType::File
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn children(&self) -> Option<&RefCell<Vec<Rc<Box<dyn Sizable>>>>> {
        None
    }
}

struct FileSystem {
    //not a perfect solution, but I wanted to stick with trait
    //objects path, and this is where it got me
    root_dir: Rc<Box<dyn Sizable>>,
    current_dir: Rc<Box<dyn Sizable>>,
}

impl FileSystem {
    fn new(dir: Rc<Box<dyn Sizable>>) -> FileSystem {
        FileSystem {
            root_dir: dir.clone(),
            current_dir: dir,
        }
    }

    fn set_current_dir(&mut self, dir: Rc<Box<dyn Sizable>>) {
        self.current_dir = dir;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut all_dirs: Vec<Rc<Box<dyn Sizable>>> = vec![];

    let root_dir = Dir::new("".to_string(), None, vec![]);
    let root_dir_rc: Rc<Box<dyn Sizable>> = Rc::new(Box::new(root_dir));
    all_dirs.push(root_dir_rc.clone());
    let mut fs = FileSystem::new(root_dir_rc);
    let mut ls_on = false;

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let command_vec: Vec<&str> = user_input.trim().split(' ').collect();
            if command_vec[0] == "$" {
                if command_vec[1] == "ls" {
                    ls_on = true;
                } else {
                    ls_on = false;
                    if command_vec[1] == "cd" {
                        match command_vec[2] {
                            ".." => {
                                fs.set_current_dir(fs.current_dir.parent().unwrap());
                            }
                            "/" => {
                                fs.set_current_dir(fs.root_dir.clone());
                            }
                            _default => {
                                let name = command_vec[2].to_string();
                                let cloned_item = {
                                    let children = fs.current_dir.children().unwrap();
                                    let children_borrowed = children.borrow();
                                    let len = fs.current_dir.children().unwrap().borrow().len();
                                    let mut inner_result = None;
                                    for i in 0..len {
                                        let item = children_borrowed.get(i).unwrap();
                                        match item.file_type() {
                                            FileType::Dir => {
                                                if *item.name() == name {
                                                    inner_result = Some(item.clone());
                                                    break;
                                                }
                                            }
                                            _default => {}
                                        }
                                    }
                                    inner_result
                                }
                                .unwrap();
                                fs.set_current_dir(cloned_item);
                            }
                        }
                    } else {
                        println!("Wrong input!");
                    }
                }
            } else if ls_on {
                if command_vec[0] == "dir" {
                    let dir = Dir::new(
                        command_vec[1].to_string(),
                        Some(Rc::downgrade(&fs.current_dir)),
                        vec![],
                    );
                    let rc_link: Rc<Box<dyn Sizable>> = Rc::new(Box::new(dir));
                    fs.current_dir
                        .children()
                        .unwrap()
                        .borrow_mut()
                        .push(rc_link.clone());
                    all_dirs.push(rc_link);
                } else {
                    let file = File {
                        size: command_vec[0].parse().unwrap(),
                        name: command_vec[0].to_string(),
                        parent_dir: Rc::downgrade(&fs.current_dir.clone()),
                    };
                    fs.current_dir
                        .children()
                        .unwrap()
                        .borrow_mut()
                        .push(Rc::new(Box::new(file)));
                }
            } else {
                println!("Wrong input!");
            }
        }
        user_input.clear();
    }

    let mut result = 0;
    let need_to_delete_size = 30000000 - (70000000 - all_dirs.first().unwrap().size());
    let mut result_part_2 = 70000000;

    for dir in all_dirs {
        let size = dir.size();
        if size <= 100000 {
            result += size;
        }
        if size >= need_to_delete_size && size < result_part_2 {
            result_part_2 = size;
        }
    }

    println!("result part 1: {}", result);
    println!("result part 2: {}", result_part_2);
}
