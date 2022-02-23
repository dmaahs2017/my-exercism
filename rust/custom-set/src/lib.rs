mod naive;
mod fast;

#[allow(unused)]
use naive::NaiveSet;
#[allow(unused)]
use fast::FastSet;
/// I may implement this in multiple ways. This is the one the tests will use.
//pub type CustomSet<T> = NaiveSet<T>;
pub type CustomSet<T> = FastSet<T, 13>;

