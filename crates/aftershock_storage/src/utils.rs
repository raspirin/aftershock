pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time error! Do we have a time machine?")
        .as_secs() as i64
}

pub type Nid = nid::Nanoid<21, Afterbet>;

pub struct Afterbet;

impl nid::alphabet::Alphabet for Afterbet {
    const SYMBOL_LIST: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";
}
