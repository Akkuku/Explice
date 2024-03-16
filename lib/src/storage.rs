pub trait Storage<T> {
    fn write(&self, item: &T) -> anyhow::Result<()>;
    fn read(&self) -> anyhow::Result<Option<T>>;
}

pub trait KVStorage<K, V>: Storage<V> {
    fn add(&self, key: K, value: V) -> anyhow::Result<()>;
    fn get(&self, key: K) -> anyhow::Result<Option<V>>;
    fn get_all(&self) -> anyhow::Result<Vec<V>>;
    fn update(&self, key: K, value: V) -> anyhow::Result<()>;
    fn delete(&self, key: K) -> anyhow::Result<()>;
}
