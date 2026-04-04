use core::fmt::Display;

/// Format an iterable of objects as RFC-822-like, with each object separated by a blank line.
///
/// The inner iterator must be `Clone` so it can be re-consumed each time `Display::fmt` is called.
pub struct Rfc822<I>(pub I);

impl<I: Clone + IntoIterator> Display for Rfc822<I>
where
    I::Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for item in self.0.clone() {
            if !first {
                writeln!(f)?;
            }

            first = false;
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_display_for_rfc822() {
        let items = vec!["item1", "item2", "item3"];
        let rfc822 = Rfc822(items);
        let output = rfc822.to_string();
        assert_eq!(output, "item1\n\nitem2\n\nitem3\n");
    }
}
