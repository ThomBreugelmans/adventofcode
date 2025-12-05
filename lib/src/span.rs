/// Defines an inclusive range of items,
/// where each item between and including `start` and `end` is container
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Span<I: Copy + Ord> {
    pub start: I,
    pub end: I,
}

impl<I> Span<I>
where
    I: Copy + Ord,
{
    /// Checks if two Spans overlap
    pub fn check_overlap(&self, other: &Span<I>) -> bool {
        (other.start.ge(&self.start) && other.end.le(&self.end))
            || (other.end.ge(&self.start) && other.end.le(&self.end))
            || (other.start.le(&self.start) && other.end.ge(&self.end))
            || (other.end.ge(&self.end) && other.start.le(&self.end))
    }
    /// Consolidates two overlapping Spans, will error if there is no overlap
    pub fn consolidate(&self, other: &Span<I>) -> Result<Span<I>, ()> {
        if !self.check_overlap(&other) {
            Err(())
        } else {
            Ok(Span {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
    /// Checks an element of type `I` if they are within the Span
    pub fn is_within(&self, element: I) -> bool {
        element.ge(&self.start) && element.le(&self.end)
    }
}

impl<I> From<(I, I)> for Span<I>
where
    I: Copy + Ord,
{
    fn from(t: (I, I)) -> Self {
        Span {
            start: t.0,
            end: t.1,
        }
    }
}

pub fn consolidate_spans<I: Copy + Ord>(mut spans: Vec<Span<I>>) -> Vec<Span<I>> {
    spans.sort();
    spans.into_iter().fold(Vec::new(), |mut a, b| {
        if a.is_empty() {
            a.push(b);
            a
        } else {
            let len_a = a.len();
            match b.consolidate(&a[a.len() - 1]) {
                Ok(new) => {
                    a[len_a - 1] = new;
                    a
                }
                Err(_) => {
                    // no overlap
                    a.push(b);
                    a
                }
            }
        }
    })
}

#[test]
fn test_ordering_of_spans() {
    let a: Span<i8> = Span { start: 4, end: 10 };
    let b: Span<i8> = Span { start: 2, end: 19 };
    let c: Span<i8> = Span { start: 2, end: 9 };
    let d: Span<i8> = Span { start: 3, end: 9 };
    let mut list = vec![a, b, d, c];
    list.sort();
    assert_eq!(list, vec![c, b, d, a]);
}
