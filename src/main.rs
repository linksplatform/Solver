use doublets::{mem, unit, Doublets, Links};
use doublets::DoubletsExt;

//         [MethodImpl(MethodImplOptions.AggressiveInlining)]
//         private ulong[] CreateAllVariants2Core(ulong[] sequence, ulong startAt, ulong stopAt)
//         {
//             if ((stopAt - startAt) == 0)
//             {
//                 return new[] { sequence[startAt] };
//             }
//             if ((stopAt - startAt) == 1)
//             {
//                 return new[] { Links.Unsync.GetOrCreate(sequence[startAt], sequence[stopAt]) };
//             }
//             var variants = new ulong[Platform.Numbers.Math.Catalan(stopAt - startAt)];
//             var last = 0;
//             for (var splitter = startAt; splitter < stopAt; splitter++)
//             {
//                 var left = CreateAllVariants2Core(sequence, startAt, splitter);
//                 var right = CreateAllVariants2Core(sequence, splitter + 1, stopAt);
//                 for (var i = 0; i < left.Length; i++)
//                 {
//                     for (var j = 0; j < right.Length; j++)
//                     {
//                         var variant = Links.Unsync.GetOrCreate(left[i], right[j]);
//                         if (variant == Constants.Null)
//                         {
//                             throw new NotImplementedException("Creation cancellation is not implemented.");
//                         }
//                         variants[last++] = variant;
//                     }
//                 }
//             }
//             return variants;
//         }

// catalan numbers
static CATALAN_NUMBERS: [usize; 25] = [
    1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845, 35357670, 129644790, 477638700, 1767263190, 6564120420, 24466267020, 91482563640, 343059613650, 1289904147324];

fn catalan_number(n: usize) -> usize {
    CATALAN_NUMBERS[n]
}

fn create_all_sequence_variants<TStore, TLinkAddress>(store: &mut TStore, sequence: &[TLinkAddress], start_at: usize, stop_at: usize) -> Result<Vec<TLinkAddress>, doublets::Error<TLinkAddress>>
where
    TLinkAddress: doublets::data::LinkType,
    TStore: Doublets<TLinkAddress>,
{
    if stop_at - start_at == 0 {
        return Ok([sequence[start_at]].to_vec());
    }
    if stop_at - start_at == 1 {
        return Ok([store.get_or_create(sequence[start_at], sequence[stop_at])?].to_vec());
    }
    let mut variants = Vec::with_capacity(catalan_number(stop_at - start_at) as usize);
    for splitter in start_at..stop_at {
        let left = create_all_sequence_variants(store, sequence, start_at, splitter)?;
        let right = create_all_sequence_variants(store, sequence, splitter + 1, stop_at)?;
        for i in 0..left.len() {
            for j in 0..right.len() {
                let variant = store.get_or_create(left[i], right[j])?;
                // if variant == TLinkAddress::funty(0) {
                //     return Err(doublets::Error::CreationCancelled);
                // }
                variants.push(variant);
            }
        }
    }
    Ok(variants)
}

fn main() -> Result<(), doublets::Error<usize>> {
    // use file as memory for doublets
    let mem = mem::FileMapped::from_path("db.links")?;
    let mut store = unit::Store::<usize, _>::new(mem)?;

    // Create a vector of N points using loop
    let n = 15;
    let mut sequence = Vec::with_capacity(n);
    for _ in 0..n {
        sequence.push(store.create_point()?);
    }

    create_all_sequence_variants(
        &mut store,
        &sequence,
        0,
        n - 1
    )?;

    // `any` constant denotes any link
    let any = store.constants().any;

    // print all store from store where (index: any, source: any, target: any)
    store.each_iter([any, any, any]).for_each(|link| {
        println!("{link:?}");
    });

    Ok(())
}