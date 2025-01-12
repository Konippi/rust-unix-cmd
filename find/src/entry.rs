use clap::ValueEnum;

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Entry {
    #[value(name = "d")]
    Dir,
    #[value(name = "f")]
    File,
    #[value(name = "l")]
    Link,
}
