mod fast;
mod naive;

pub use fast::FastSet;
pub use naive::NaiveSet;
/// I may implement this in multiple ways. This is the one the tests will use.
//pub type CustomSet<T> = NaiveSet<T>;
pub type CustomSet<T> = FastSet<T, 13>;
