pub trait Persist
where
    Self: Sized,
{
    fn read() -> anyhow::Result<Self>;
    fn save(&self) -> anyhow::Result<()>;
}
