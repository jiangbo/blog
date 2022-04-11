fn get_max<F>(&mut self) -> Option<T> {
        // match self.root {
        //     Some(_) => {
        //         let mut current = &mut self.root;
        //         while current.as_ref().unwrap().right.is_some() {
        //             current = &mut current.as_mut().unwrap().right
        //         }
        //         let temp = current.take().unwrap();
        //         *current = temp.left;
        //         Some(temp.value)
        //     }

        //     None => None,
        // }

        let mut current = &mut self.root;
        while let Some(node) = current.as_mut() {
            current = match node.as_mut().right {
                Some(_) => node.as_mut().right,
                None => return Some(&node.value),
            }
        }
        None
    }