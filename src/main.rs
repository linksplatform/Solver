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
  render_visited: bool,
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
    &append_index,
    render_visited,
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
  append_index: &impl Fn(&mut String, T, bool, bool, bool),
  render_visited: bool,
  render_index: bool,
  render_debug: bool,
) -> Result<(), Error<T>>
where
  T: LinkType,
  S: Doublets<T>,
{
  let constants = store.constants();
  if [constants.null, constants.any, constants.itself].contains(&link_index) {
    return Ok(());
  }

  let mut is_missing = !store.exist(link_index);
  let is_visited = !visited.insert(link_index);

  // Skip fetching the link if it's missing or visited
  if is_missing || (is_visited && !render_visited) {
    append_index(sb, link_index, is_missing, is_visited, render_debug);
    return Ok(());
  }

  // Call get_link to check if the link exists
  let link = store.get_link(link_index);
  is_missing = link.is_none();

  if is_missing {
    append_index(sb, link_index, is_missing, is_visited, render_debug);
    return Ok(());
  }

  let link = link.unwrap();

  // Check if the link is an element after unwrapping
  if is_element(&link) {
    append_index(sb, link_index, is_missing, is_visited, render_debug);
    return Ok(());
  }

  // Open the structure with '('
  sb.push('(');

  // Render index if required
  if render_index {
    append_index(sb, link_index, is_missing, is_visited, render_debug);
    sb.push(':');
    sb.push(' ');
  }

  // Recur for source and target
  append_structure(store, sb, visited, link.source, is_element, append_index, render_visited, render_index, render_debug)?;
  sb.push(' ');
  append_structure(store, sb, visited, link.target, is_element, append_index, render_visited, render_index, render_debug)?;

  // Close the structure with ')'
  sb.push(')');

  Ok(())
}

fn append_index<T>(
  sb: &mut String,
  index: T,
  is_missing: bool,
  is_visited: bool,
  render_debug: bool,
) 
where
  T: LinkType,
{
  if render_debug {
    if is_missing {
      sb.push('~');
    } else if is_visited {
      sb.push('*');
    }
  }

  // Always render the index at the end
  write!(sb, "{}", index).unwrap();
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
  let max_seq_length = 4; // Change this as needed

  // Generate all possible sequences of `1` and `2` with the specified length
  let sequences: Vec<Vec<usize>> = (1..=max_seq_length)
    .flat_map(|length| args.iter().cloned().combinations_with_replacement(length))
    .collect();

  println!("Total sequences: {}", sequences.len());
  for seq in &sequences {
    let mut seq_string = format!("{:?}", seq);
    seq_string = seq_string.replace(&x.to_string(), "x");
    seq_string = seq_string.replace(&y.to_string(), "y");
    println!("{}", seq_string);
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
      let mut deep_structure = deep_format(&mut store, *variant, |link| link.is_partial(), true, false, false)?;
      deep_structure = deep_structure.replace(&x.to_string(), "x");
      deep_structure = deep_structure.replace(&y.to_string(), "y");
      println!("{deep_structure}");
    }
  }

  // `any` constant denotes any link
  let any = store.constants().any;

  println!("Total links: {}", store.count());
  store.each_iter([any, any, any]).for_each(|link| {
    println!("{link:?}");
  });

  // println!("Check for full points:");
  // // Iterate over all links and check if they are full points
  // store.each_iter([any, any, any]).for_each(|link| {
  //   println!("{:?} is a full point: {}", link, link.is_full());
  // });

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
