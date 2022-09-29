# slice-rbtree
A `#[no_std]` [Red-Black tree][2], fully packed in a single slice of bytes
Originally developed for storing data in [Solana][0] [Accounts][1], this crate allows you to
access tree nodes without deserializing the whole tree. It is useful when you have a huge
tree in raw memory, but want to interact only with a few values at a time.

There are two core type in this crate: `RBTree` and `RBForest`

## `RBTree`
As name suggests, it is a [Red-Black tree][2], contained in the slice of bytes.
The API is similar to [BTreeMap][3] with a few exceptions, such as [Entry API][4], but it will be added in the future releases.
```rust
use slice_rbtree::{tree_size, RBTree};
// RBTree requires input slice to have a proper size
// Each node in the `RBTree` has a fixed size known at compile time, so to estimate this size `KSIZE` and `VSIZE` parameters should passed to forest_size
let size = tree_size(50, 50, 10, 2);
let mut buffer = vec![0; size];
// `String` type has variable length, but we have to chose some fixed maximum length (50 bytes for both key and value)
let mut reviews: RBTree<String, String, 50, 50> = RBForest::init_slice(&mut buffer, 2).unwrap();

// Let tree 0 be the movie tree and tree 1 - the book tree

// review some movies.
reviews.insert(0,"Office Space".to_string(),       "Deals with real issues in the workplace.".to_string());
reviews.insert(0,"Pulp Fiction".to_string(),       "Masterpiece.".to_string());
reviews.insert(0,"The Godfather".to_string(),      "Very enjoyable.".to_string());
reviews.insert(0,"The Blues Brothers".to_string(), "Eye lyked it a lot.".to_string());

// review some books
reviews.insert(1,"Fight club".to_string(),       "Brad Pitt is cool!".to_string());
reviews.insert(1,"Alice in Wonderland".to_string(),       "Deep than you think.".to_string());
reviews.insert(1,"1984".to_string(),      "A scary dystopia.".to_string());
reviews.insert(1,"The Lord of the Rings".to_string(), "Poor Gollum.".to_string());

// check for a specific one.
if !reviews.contains_key(0,"Les Misérables") {
    println!("We've got {} reviews, but Les Misérables ain't one.",
             reviews.len(0));
}

// oops, this review has a lot of spelling mistakes, let's delete it.
reviews.remove(0, "The Blues Brothers");

// look up the values associated with some keys.
let to_find = ["Up!".to_string(), "Office Space".to_string()];
for movie in &to_find {
    match reviews.get(movie) {
       Some(review) => println!("{movie}: {review}"),
       None => println!("{movie} is unreviewed.")
    }
}

// iterate over everything.
for (movie, review) in reviews.pairs() {
    println!("{movie}: \"{review}\"");
}
```
## `RBforest`
It sometimes happens, that you have to use a set of similar trees of unknown size. In that
case you could allocate such trees in different slices, but it will be very ineffective: you
have to think about capacity of each tree beforehand and it is still possible, that some trees
will be full, while others are (almost) empty.

`RBForest` solves this issue, by using a common node pool for a set of trees.
The API of [`RBForest`] mimics [`RBTree`](super::RBTree) but with one additional argument: index of the tree.
```rust
use slice_rbtree::{forest_size, RBForest};
// RBTree requires input slice to have a proper size
// Each node in the `RBTree` has a fixed size known at compile time, so to estimate this size `KSIZE` and `VSIZE` parameters should passed to forest_size
let size = forest_size(50, 50, 10, 2);
let mut buffer = vec![0; size];
// `String` type has variable length, but we have to chose some fixed maximum length (50 bytes for both key and value)
let mut reviews: RBForest<String, String, 50, 50> = RBForest::init_slice(&mut buffer, 2).unwrap();

// Let tree 0 be the movie tree and tree 1 - the book tree

// review some movies.
reviews.insert(0,"Office Space".to_string(),       "Deals with real issues in the workplace.".to_string());
reviews.insert(0,"Pulp Fiction".to_string(),       "Masterpiece.".to_string());
reviews.insert(0,"The Godfather".to_string(),      "Very enjoyable.".to_string());
reviews.insert(0,"The Blues Brothers".to_string(), "Eye lyked it a lot.".to_string());

// review some books
reviews.insert(1,"Fight club".to_string(),       "Brad Pitt is cool!".to_string());
reviews.insert(1,"Alice in Wonderland".to_string(),       "Deep than you think.".to_string());
reviews.insert(1,"1984".to_string(),      "A scary dystopia.".to_string());
reviews.insert(1,"The Lord of the Rings".to_string(), "Poor Gollum.".to_string());

// check for a specific one.
if !reviews.contains_key(0,"Les Misérables") {
    println!("We've got {} movie reviews, but Les Misérables ain't one.",
             reviews.len(0));
}
if reviews.contains_key(1,"1984") {
    println!("We've got {} book reviews and 1984 among them: {}.",
             reviews.len(0), reviews.get(1, "1984").unwrap());
}

// oops, this review has a lot of spelling mistakes, let's delete it.
reviews.remove(0, "The Blues Brothers");

// look up the values associated with some keys.
let to_find = ["Up!".to_string(), "Office Space".to_string()];
for movie in &to_find {
    match reviews.get(0, movie) {
       Some(review) => println!("{movie}: {review}"),
       None => println!("{movie} is unreviewed.")
    }
}

// iterate over movies.
for (movie, review) in reviews.pairs(0) {
    println!("{movie}: \"{review}\"");
}
///
// Too many reviews, delete them all!
reviews.clear();
assert!(reviews.is_empty(0));
assert!(reviews.is_empty(1));
```
[0]: https://docs.solana.com/
[1]: https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html
[2]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
[3]: https://doc.rust-lang.org/stable/std/collections/btree_map/struct.BTreeMap.html
[4]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html#method.entry
