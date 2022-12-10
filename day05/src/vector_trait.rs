pub trait AodVecTrait<T: Default> {
    //fn get_or_default(&mut self, index: usize) -> &T;
    fn get_or_default_mut(&mut self, index: usize) -> &mut T;
}

impl<T: Default> AodVecTrait<T> for Vec<T> {
    //fn get_or_default(&mut self, index: usize) -> &T {
    //    if self.len() > index {
    //        return &self[index];
    //    }
    //
    //    self.push(T::default());
    //    self.last().unwrap()
    //}

    fn get_or_default_mut(&mut self, index: usize) -> &mut T {
        if self.len() > index {
            return &mut self[index];
        }

        self.push(T::default());
        self.last_mut().unwrap()
    }
}
