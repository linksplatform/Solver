use doublets::{
  data::LinkType, mem, mem::RawMem, unit, unit::LinkPart, Doublets, DoubletsExt, Error, Link, Links,
};
use std::{collections::HashSet, fmt::Write};
use tap::Pipe;
use itertools::Itertools;

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

fn get_link_by_id<T>(
  store: &mut unit::Store<usize, T>,
  id: usize,
) -> Result<Link<usize>, Error<usize>>
where
  T: RawMem<LinkPart<usize>>,
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

pub fn deep_format<T, S>(
  store: &mut S,
  link_index: T,
  is_element: impl Fn(&Link<T>) -> bool,
  render_index: bool,
  render_debug: bool,
) -> Result<String, Error<T>>
where
  T: LinkType,
  S: Doublets<T>,
{
  let mut sb = String::new();
  let mut visited = HashSet::new();
  append_structure(
    store,
    &mut sb,
    &mut visited,
    link_index,
    &is_element,
    &|sb, link| {
      write!(sb, "{}", link.index).unwrap();
    },
    render_index,
    render_debug,
  )?;
  Ok(sb)
}

fn append_structure<T, S>(
  store: &mut S,
  sb: &mut String,
  visited: &mut HashSet<T>,
  link_index: T,
  is_element: &impl Fn(&Link<T>) -> bool,
  append_element: &impl Fn(&mut String, &Link<T>),
  render_index: bool,
  render_debug: bool,
) -> Result<(), Error<T>>
where
  T: LinkType,
  S: Doublets<T>,
{
  let constants = store.constants();
  if link_index == constants.null || link_index == constants.any || link_index == constants.itself {
    return Ok(());
  }

  if store.exist(link_index) {
    if visited.insert(link_index) {
      sb.push('(');
      let link = store.get_link(link_index).unwrap();

      if render_index {
        write!(sb, "{}: ", link.index).unwrap();
      }

      if link.source == link.index {
        write!(sb, "{}", link.index).unwrap();
      } else {
        let source = store.get_link(link.source).unwrap();
        if is_element(&source) {
          append_element(sb, &source);
        } else {
          append_structure(
            store,
            sb,
            visited,
            source.index,
            is_element,
            append_element,
            render_index,
            render_debug,
          )?;
        }
      }

      sb.push(' ');

      if link.target == link.index {
        write!(sb, "{}", link.index).unwrap();
      } else {
        let target = store.get_link(link.target).unwrap();
        if is_element(&target) {
          append_element(sb, &target);
        } else {
          append_structure(
            store,
            sb,
            visited,
            target.index,
            is_element,
            append_element,
            render_index,
            render_debug,
          )?;
        }
      }

      sb.push(')');
    } else {
      if render_debug {
        sb.push('*');
      }
      write!(sb, "{}", link_index).unwrap();
    }
  } else {
    if render_debug {
      sb.push('~');
    }
    write!(sb, "{}", link_index).unwrap();
  }
  Ok(())
}

fn main() -> Result<(), Error<usize>> {
  let mem = mem::Global::new();
  let mut store = unit::Store::<usize, _>::new(mem)?;

  let link_type = store.create_point()?;

  let x = store.create_point()?;
  store.update(x, x, link_type);
  let y = store.create_point()?;
  store.update(y, y, link_type);

  // Define the two links
  let args = vec![x, y];

  // Specify the length of the sequences you want (e.g., 1 to 16)
  let seq_length = 2; // Change this as needed

  // Generate all possible sequences of `1` and `2` with the specified length
  let sequences: Vec<Vec<usize>> = (1..=seq_length)
    .flat_map(|length| args.iter().cloned().combinations_with_replacement(length))
    .collect();

  println!("Total sequences: {}", sequences.len());
  for seq in &sequences {
    println!("{:?}", seq);
  }

  // Use the generated sequences to create variants
  for seq in sequences {
    let result = all_seq_variants(&mut store, &seq)?;

    println!("Total variants: {}", result.len());
    for variant in &result {
      println!("{variant}");
    }

    println!("Full structure:");
    for variant in &result {
      let deep_structure = deep_format(&mut store, *variant, |link| link.is_partial(), true, true)?;
      println!("{deep_structure}");
    }
  }

  // `any` constant denotes any link
  let any = store.constants().any;

  println!("Total links: {}", store.count());
  store.each_iter([any, any, any]).for_each(|link| {
    println!("{link:?}");
  });

  println!("Check for full points:");
  // Iterate over all links and check if they are full points
  store.each_iter([any, any, any]).for_each(|link| {
    println!("{:?} is a full point: {}", link, link.is_full());
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
