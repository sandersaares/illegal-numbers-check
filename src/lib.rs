use std::sync::LazyLock;

pub static ILLEGAL_NUMBERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    const ILLEGAL_NUMBER_START: usize = 5_000_000;
    const ILLEGAL_NUMBER_COUNT: usize = 10_000_000;

    let mut numbers = Vec::with_capacity(ILLEGAL_NUMBER_COUNT);

    // 10M numbers is something like 70 MB of characters alone (plus String overhead etc).
    // With overhead included, it comes to around 390 MB (that is some big overhead).
    //
    // For our purposes, we just want to use a large data set for easy demonstration of
    // large data set effects (which in real world apps might be more "many smaller data sets"
    // that total a large amount of data).
    for i in ILLEGAL_NUMBER_START..(ILLEGAL_NUMBER_START + ILLEGAL_NUMBER_COUNT) {
        numbers.push(i.to_string());
    }

    numbers
});
