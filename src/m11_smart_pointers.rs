#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointer() {
        // // Box - stored heap
        // let x: Box<i32> = Box::new(50);

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let nodes: Box<Node> = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node {
                id: 1,
                next: Some(Box::new(Node { id: 2, next: None })),
            })),
        });

        dbg!(nodes);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {
        // Reference Counter
        // 値がまだ使用中かどうか決定する値への参照の数を追跡。参照されるとカウントが増加
        // 値への参照が0なら、どの参照も無効にすることなく、 値は片付けられる。
        // RefCell - 動的にチェックされる借用ルールを持つ、変更可能なメモリロケーション
        let x = Rc::new(RefCell::new(50));
        let y = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x.borrow_mut());
        dbg!(y.borrow_mut());

        #[derive(Debug, Clone)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }

        let mut house_1 = Rc::new(House {
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec![]),
        });

        let table = Rc::new(Furniture {
            id: "table".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1),
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1),
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);
    }
}
