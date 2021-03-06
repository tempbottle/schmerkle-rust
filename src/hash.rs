use std::hash::Hasher;

// Standard Hasher trait is constrained to 64bit hashes
// by finish() method.
// MerkleHasher extends Hasher to be a size-agnostic alternative.
// Also MerkleHasher works with std::hash::Hash (yay!).
pub trait MerkleHasher: Hasher
{
    // Just like finish, but not constrained to 64bits
    fn finish_full(&self) -> Box<[u8]>;
}


// Analogous to std::hash::BuildHasher
pub trait BuildMerkleHasher : Clone {
    type Hasher: MerkleHasher;
    fn build_hasher(&self) -> Self::Hasher;
}