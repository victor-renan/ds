use std::ptr;

struct NodeTree {
    value: i32,
    left: *const NodeTree,
    right: *const NodeTree,
}

impl NodeTree {
    unsafe fn traverse_preorder(&self) {
        print!("{}", self.value);

        if !self.left.is_null() {
            print!("->");
            (*self.left).traverse_preorder();
        }

        if !self.right.is_null() {
            print!("->");
            (*self.right).traverse_preorder();
        }

        return;
    }
}

fn main() {
    let teste: NodeTree = NodeTree {
        value: 1,
        left: &NodeTree {
            value: 2,
            left: &NodeTree {
                value: 3,
                left: &NodeTree {
                    value: 4,
                    left: ptr::null(),
                    right: ptr::null(),
                },
                right: ptr::null(),
            },
            right: &NodeTree {
                value: 5,
                left: ptr::null(),
                right: ptr::null(),
            }
        },
        right: &NodeTree {
            value: 6,
            left: &NodeTree {
                value: 7,
                left: ptr::null(),
                right: ptr::null(),
            },
            right: ptr::null(),
        },
    };

    unsafe {
        print!("\nPré "); 
        teste.traverse_preorder();
    }
}
