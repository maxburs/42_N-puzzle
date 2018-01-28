use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;

use super::Expandable;

#[derive(Clone, Eq, PartialEq)]
pub struct State<T: Expandable + Hash> {
    pub data: T,
    pub previous: Option<Rc<RefCell<State<T>>>>,
    pub distance: usize,
    pub open: bool,
}

pub fn new<T: Expandable + Hash>(data: T, distance: usize) -> State<T> {
    State {
        data,
        previous: None,
        distance,
        open: true,
    }
}

impl<T: Expandable + Hash> Expandable for State<T> {
    fn expand(&self) -> Vec<Self> {
        self.data
            .expand()
            .into_iter()
            .map(|data|
                State {
                    data,
                    previous: None,
                    distance: self.distance + 1,
                    open: true,
                }
            )
            .collect()
    }
}

// #[test]
// fn test_expand() {
//     let base = Rc::new(RefCell::new(
//         State {
//             data: vec![
//                 vec![1, 0],
//                 vec![1, 1],
//             ],
//             x: 1,
//             y: 0,
//             distance: 0,
//             open: false,
//             previous: None
//         }
//     ));

//     assert_eq!(
//         vec![
//             State {
//                 data: vec![
//                     vec![0, 1],
//                     vec![1, 1],
//                 ],
//                 x: 0,
//                 y: 0,
//                 distance: 1,
//                 open: true,
//                 previous: None,
//             },
//             State {
//                 data: vec![
//                     vec![1, 1],
//                     vec![1, 0],
//                 ],
//                 x: 1,
//                 y: 1,
//                 distance: 1,
//                 open: true,
//                 previous: None,
//             }
//         ],
//         base.borrow().expand()
//     );
// }
