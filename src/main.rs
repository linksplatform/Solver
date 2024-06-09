use doublets::mem::RawMem;
use doublets::{data::LinkType, mem, unit, Doublets, Error};
use doublets::{DoubletsExt, Link, Links};
use doublets::unit::LinkPart;
use tap::Pipe;

#[rustfmt::skip]
const CATALAN_NUMBERS: [u64; 25] = [
    1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012,
    742900, 2674440, 9694845, 35357670, 129644790,  477638700, 1767263190,
    6564120420, 24466267020, 91482563640, 343059613650, 1289904147324,
];

const fn catalan(n: usize) -> u64 {
    CATALAN_NUMBERS[n]
}

fn spec_all_variants<T, S>(store: &mut S, seq: &[T]) -> Result<Vec<T>, Error<T>>
where
    T: LinkType,
    S: Doublets<T>,
{
    assert!(seq.len() > 2);

    let mut variants = Vec::with_capacity(catalan(seq.len() - 1) as usize);
    for splitter in 1..seq.len() {
        let (left, right) = seq.split_at(splitter);
        let (left, right) = (
            all_seq_variants(store, left)?,
            all_seq_variants(store, right)?,
        );
        for from in left {
            for &to in &right {
                variants.push(store.get_or_create(from, to)?);
            }
        }
    }
    Ok(variants)
}

fn all_seq_variants<T, S>(store: &mut S, seq: &[T]) -> Result<Vec<T>, Error<T>>
where
    T: LinkType,
    S: Doublets<T>,
{
    match seq {
        &[single] => {
            vec![single]
        }
        &[from, to] => {
            vec![store.get_or_create(from, to)?]
        }
        seq => spec_all_variants(store, seq)?,
    }
    .pipe(Ok)
}

/// Performs a NAND operation on two boolean inputs.
///
/// # Arguments
///
/// * `a` - A boolean input.
/// * `b` - A boolean input.
///
/// # Returns
///
/// * A boolean output representing the NAND operation on the inputs.
fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn get_link_by_id<T>(store: &mut unit::Store<usize, T>, id: usize) -> Result<Link<usize>, Error<usize>>
where
    T: RawMem<LinkPart<usize>>
{
    // `any` constant denotes any link
    let any = store.constants().any;
    let mut link_result = Err(Error::NotExists(id));

    store.each_iter([id, any, any]).for_each(|link| {
        if link.index == id {
            link_result = Ok(link);
        }
    });

    link_result
}

fn main() -> Result<(), Error<usize>> {
    let mem = mem::Global::new();
    let mut store = unit::Store::<usize, _>::new(mem)?;

    // Create a vector of N points using iterators
    let seq: Vec<_> = (0..3)
        .map(|_| store.create_point())
        .collect::<Result<_, _>>()?;

    let result = all_seq_variants(&mut store, &seq)?;
    println!("{}", result.len());
    println!("{}", result[0]);

    // `any` constant denotes any link
    let any = store.constants().any;

    store.each_iter([any, any, any]).for_each(|link| {
        println!("{link:?}");
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(true, true), false);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(false, false), true);
    }
}
