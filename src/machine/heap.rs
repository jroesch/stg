pub trait Heap<K, V> {
    fn extend(&mut self, bindings: Vec<(K, V)>);
    fn value_for(&self, key: K) -> Option<V>;
    fn allocate(&mut self, obj: V) -> K;
}
