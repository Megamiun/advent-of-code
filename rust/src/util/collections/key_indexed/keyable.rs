pub trait Keyable {
    type Key: Ord;
    
    fn get_key(&self) -> Self::Key;
}

impl<K: Ord + Copy, T> Keyable for (K, T) {
    type Key = K;

    fn get_key(&self) -> K {
        self.0
    }
}

