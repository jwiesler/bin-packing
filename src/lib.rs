#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct NotPossible;

pub trait Item {
    fn size(&self) -> usize;
}

impl Item for usize {
    fn size(&self) -> usize {
        *self
    }
}

impl<T> Item for (usize, T) {
    fn size(&self) -> usize {
        self.0
    }
}

pub fn bin_pack<I: Item + Clone>(
    values: &[I],
    free_space: &mut [usize],
) -> Result<Vec<Vec<I>>, NotPossible> {
    let Some((to_fit, rest)) = values.split_last() else {
        return Ok(vec![Vec::new(); free_space.len()]);
    };
    for bin in (0..free_space.len()).rev() {
        if to_fit.size() <= free_space[bin] {
            free_space[bin] -= to_fit.size();
            if let Ok(mut assignment) = bin_pack(rest, free_space) {
                assignment[bin].push(to_fit.clone());
                return Ok(assignment);
            }
            free_space[bin] += to_fit.size();
        }
    }
    Err(NotPossible)
}

#[cfg(test)]
mod tests {
    use super::{bin_pack, NotPossible};

    #[test]
    fn test_bin_pack() {
        assert_eq!(bin_pack::<usize>(&[], &mut []), Ok(vec![]));
        assert_eq!(bin_pack::<usize>(&[], &mut [1]), Ok(vec![vec![]]));
        assert_eq!(
            bin_pack(&[(1, "a"), (2, "b"), (3, "c")], &mut [1, 2, 3]),
            Ok(vec![vec![(1, "a")], vec![(2, "b")], vec![(3, "c")]])
        );
        assert_eq!(
            bin_pack(
                &[
                    (3, "a"),
                    (3, "b"),
                    (3, "c"),
                    (3, "d"),
                    (3, "e"),
                    (3, "f"),
                    (3, "g"),
                    (3, "h")
                ],
                &mut [8, 8, 8]
            ),
            Err(NotPossible)
        );
        assert_eq!(
            bin_pack(
                &[
                    (3, "a"),
                    (3, "b"),
                    (2, "c"),
                    (3, "d"),
                    (3, "e"),
                    (2, "f"),
                    (3, "g"),
                    (3, "h"),
                    (2, "i"),
                ],
                &mut [8, 8, 8]
            ),
            Ok(vec![
                vec![(3, "a"), (3, "b"), (2, "c")],
                vec![(3, "d"), (3, "e"), (2, "f")],
                vec![(3, "g"), (3, "h"), (2, "i")]
            ]),
        );
    }
}

