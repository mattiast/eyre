mod drop;

use self::drop::{DetectDrop, Flag};
use eyre::{Report, Result};
use std::error::Error as StdError;

#[test]
fn test_convert() {
    let has_dropped = Flag::new();
    let error: Report = Report::new(DetectDrop::new(&has_dropped));
    let box_dyn = Box::<dyn StdError + Send + Sync>::from(error);
    assert_eq!("oh no!", box_dyn.to_string());
    drop(box_dyn);
    assert!(has_dropped.get());
}

#[test]
fn test_question_mark() -> Result<(), Box<dyn StdError>> {
    fn f() -> Result<()> {
        Ok(())
    }
    f()?;
    Ok(())
}
