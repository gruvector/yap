use crate::Tokens;

/// Produced by running [`crate::tokens::Tokens::slice`].
pub struct Slice<'a, T: Tokens> {
    tokens: &'a mut T,
    original: T::Location,
    started: bool,
    from: T::Location,
    to: T::Location,
}

impl<'a, T: Tokens> Slice<'a, T> {
    pub(crate) fn new(
        tokens: &'a mut T,
        current: T::Location,
        from: T::Location,
        to: T::Location,
    ) -> Slice<'a, T> {
        Slice {
            tokens,
            original: current,
            from,
            to,
            started: false,
        }
    }
}

// We can also treat this slice of tokens as tokens, too:
impl<'a, T: Tokens> Tokens for Slice<'a, T> {
    type Item = T::Item;
    type Location = T::Location;

    fn next(&mut self) -> Option<Self::Item> {
        // Initial prep:
        if !self.started {
            self.tokens.set_location(self.from.clone());
            self.started = true;
        }

        // Once we hit the "to" location we return None
        // (this isn't an inclusive iterator).
        if self.tokens.is_at_location(&self.to) {
            None
        } else {
            self.tokens.next()
        }
    }
    fn location(&self) -> Self::Location {
        self.tokens.location()
    }
    fn set_location(&mut self, location: Self::Location) {
        self.tokens.set_location(location)
    }
    fn is_at_location(&self, location: &Self::Location) -> bool {
        self.tokens.is_at_location(location)
    }

    // Delegate to `parse_slice` here, since that function can be optimised
    // by specific implementations.
    fn parse<Out, Buf>(&mut self) -> Result<Out, <Out as core::str::FromStr>::Err>
    where
        Out: core::str::FromStr,
        Buf: FromIterator<Self::Item> + core::ops::Deref<Target = str>,
    {
        let res = self
            .tokens
            .parse_slice::<Out, Buf>(self.from.clone(), self.to.clone())?;

        // If parse was successful, consume the rest of this Slice, to be
        // consistent with how `parse` normally works.
        self.tokens.set_location(self.to.clone());
        Ok(res)
    }
}

impl<'a, T: Tokens> Drop for Slice<'a, T> {
    fn drop(&mut self) {
        // Reset the location on drop so that the tokens
        // remain unaffected by this iterator:
        self.tokens.set_location(self.original.clone());
    }
}
