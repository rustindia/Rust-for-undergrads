
// Here we have two sequences of data. These could be stored in vectors
// or linked lists or whatever. Here we have _slices_ (references to arrays):
let data1 = &[3, 1, 4, 1, 5, 9, 2, 6];
let data2 = &[2, 7, 1, 8, 2, 8, 1, 8];
  
// Let’s compute some valuable results from them!
let numbers =
    // By iterating over the first array:
    data1.iter()            // {3,      1,      4,      ...}
    // Then zipping this iterator with an iterator over another array,
    // resulting in an iterator over pairs of numbers:
    .zip(data2.iter())      // {(3, 2), (1, 7), (4, 1), ...}
    // After that we map each pair into the product of its elements
    // via a lambda function and get an iterator over products:
    .map(|(a, b)| a * b)    // {6,      7,      4,      ...}
    // Given that, we filter some of the results with a predicate:
    .filter(|n| *n > 5)     // {6,      7,              ...}
    // And take no more than 4 of the entire sequence which is produced
    // by the iterator constructed to this point:
    .take(4)
    // Finally, we collect the results into a vector. This is
    // the point where the iteration is actually performed:
    .collect::<Vec<_>>();
  
// And here is what we can see if we print out the resulting vector:
println!("{:?}", numbers);  // ===> [6, 7, 8, 10]
