use std::cell::RefCell;
use std::io;

struct Tree {
    height: u8,
    visibility: Vec<bool>, //left, top, right, bottom
}

impl Tree {
    fn new(height: u8) -> Tree {
        Tree {
            height,
            visibility: vec![false; 4],
        }
    }

    fn visible(&self) -> bool {
        self.visibility.iter().any(|&vis| vis)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut tree_rows: Vec<Vec<RefCell<Tree>>> = vec![];

    while let Ok(bytes) = stdin.read_line(&mut user_input) {
        if bytes == 0 {
            break;
        } else {
            let mut row = vec![];
            let trimmed = user_input.trim();
            let chars = trimmed.chars();
            for ch in chars {
                let height: u8 = ch.to_string().parse().unwrap();
                let tree = Tree::new(height);
                row.push(RefCell::new(tree));
            }
            tree_rows.push(row);
            user_input.clear();
        }
    }

    //part 1

    //left
    for row in &tree_rows {
        let mut biggest_height = 0;
        for (index, tree) in row.iter().enumerate() {
            if index == 0 || tree.borrow().height > biggest_height {
                tree.borrow_mut().visibility[0] = true;
            }
            if tree.borrow().height > biggest_height {
                biggest_height = tree.borrow().height;
            }
        }
    }

    //top
    for x in 0..tree_rows.first().unwrap().len() {
        let mut biggest_height = 0;
        for y in 0..tree_rows.len() {
            let tree = tree_rows.get(y).unwrap().get(x).unwrap();
            if tree.borrow().visible() {
                if tree.borrow().height > biggest_height {
                    biggest_height = tree.borrow().height;
                }
                continue;
            }
            if y == 0 || tree.borrow().height > biggest_height {
                tree.borrow_mut().visibility[1] = true;
                biggest_height = tree.borrow().height;
            }
        }
    }

    //right
    for row in &tree_rows {
        let mut biggest_height = 0;
        for (index, tree) in row.iter().rev().enumerate() {
            if tree.borrow().visible() {
                if tree.borrow().height > biggest_height {
                    biggest_height = tree.borrow().height;
                }
                continue;
            }
            if index == 0 || tree.borrow().height > biggest_height {
                tree.borrow_mut().visibility[2] = true;
                biggest_height = tree.borrow().height;
            }
        }
    }

    //bottom
    let mut result = 0;
    for x in 0..tree_rows.first().unwrap().len() {
        let mut biggest_height = 0;
        for y in 0..tree_rows.len() {
            let tree = tree_rows
                .get(tree_rows.len() - y - 1)
                .unwrap()
                .get(x)
                .unwrap();
            if tree.borrow().visible() {
                result += 1;
                if tree.borrow().height > biggest_height {
                    biggest_height = tree.borrow().height;
                }
                continue;
            }
            if y == 0 || tree.borrow().height > biggest_height {
                tree.borrow_mut().visibility[3] = true;
                biggest_height = tree.borrow().height;
                result += 1;
            }
        }
    }

    //part 2
    let mut highest_score = 0;
    for (y, row) in tree_rows.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            //left
            let mut left_view = 0;
            for x_inner in (0..x).rev() {
                let tree_inner = tree_rows.get(y).unwrap().get(x_inner).unwrap();
                left_view += 1;
                if tree_inner.borrow().height >= tree.borrow().height {
                    break;
                }
            }

            //top
            let mut top_view = 0;
            for y_inner in (0..y).rev() {
                let tree_inner = tree_rows.get(y_inner).unwrap().get(x).unwrap();
                top_view += 1;
                if tree_inner.borrow().height >= tree.borrow().height {
                    break;
                }
            }

            //right
            let mut right_view = 0;
            for x_inner in (x + 1)..row.len() {
                let tree_inner = tree_rows.get(y).unwrap().get(x_inner).unwrap();
                right_view += 1;
                if tree_inner.borrow().height >= tree.borrow().height {
                    break;
                }
            }

            //bottom
            let mut bottom_view = 0;
            for y_inner in (y + 1)..tree_rows.len() {
                let tree_inner = tree_rows.get(y_inner).unwrap().get(x).unwrap();
                bottom_view += 1;
                if tree_inner.borrow().height >= tree.borrow().height {
                    break;
                }
            }

            let score = left_view * top_view * right_view * bottom_view;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("result part 1: {}", result);
    println!("result part 2: {}", highest_score);
}
