use std::ptr;

struct NodeTree {
    value: i32,
    left: *const NodeTree,
    right: *const NodeTree,
}

impl NodeTree {
    //
}

fn main() {
    let mut x: NodeTree = NodeTree {
        value: 1,
        left: ptr::null(),
        right: ptr::null(),
    };

    let mut y: NodeTree = NodeTree {
        value: 2,
        left: ptr::null(),
        right: ptr::null(),
    };

    let z: NodeTree = NodeTree {
        value: 3,
        left: ptr::null(),
        right: ptr::null(),
    };

    x.left = &y;
    x.right = &z;

    unsafe {
        print!("value: {}\nleft: {}\nright: {}\n",
            x.value, (*x.left).value, (*x.right).value);
    }
}
