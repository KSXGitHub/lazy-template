use crate::Template;
use core::{convert::Infallible, marker::PhantomData, ops::Deref};
use derive_more::IntoIterator;

#[derive(Debug, Clone, Copy)]
pub struct EagerParseIter<SegmentIter>(SegmentIter);

impl<SegmentIter> Iterator for EagerParseIter<SegmentIter>
where
    SegmentIter: Iterator,
    SegmentIter::Item: Deref,
    <SegmentIter::Item as Deref>::Target: Copy,
{
    type Item = Result<<SegmentIter::Item as Deref>::Target, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_deref().copied().map(Ok)
    }
}

#[derive(Debug, Clone, Copy, IntoIterator)]
pub struct ParsedTemplate<SegmentContainer, Query> {
    #[into_iterator]
    container: SegmentContainer,
    _query: PhantomData<Query>, // phantom Query is necessary to enable type inference later on
}

impl<SegmentContainer, Query> ParsedTemplate<SegmentContainer, Query> {
    pub(crate) fn new(container: SegmentContainer) -> Self {
        Self {
            container,
            _query: PhantomData,
        }
    }
}

impl<'a, SegmentContainer, Query> ParsedTemplate<SegmentContainer, Query>
where
    SegmentContainer: Deref + 'a,
    &'a SegmentContainer::Target: IntoIterator,
{
    pub fn to_template(
        &'a self,
    ) -> Template<EagerParseIter<<&'a SegmentContainer::Target as IntoIterator>::IntoIter>, Query>
    {
        Template::new(self.iter())
    }

    pub fn iter(
        &'a self,
    ) -> EagerParseIter<<&'a SegmentContainer::Target as IntoIterator>::IntoIter> {
        EagerParseIter(self.container.into_iter())
    }
}
