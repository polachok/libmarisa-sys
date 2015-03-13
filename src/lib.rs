extern crate libc;
use libc::{c_char, c_int, size_t};

#[repr(C)] pub struct KeySet;
#[repr(C)] pub struct Trie;
#[repr(C)] pub struct Key;
#[repr(C)] pub struct Agent;

extern "C" {
    pub fn keyset_create() -> *const KeySet;
    pub fn keyset_push(keyset: *const KeySet, string: *const c_char, len: c_int, val: c_int);
    pub fn keyset_destroy(keyset: *const KeySet);
    pub fn trie_create() -> *const Trie;
    pub fn trie_build(trie: *const Trie, keyset: *const KeySet, options: c_int);
    pub fn trie_save(trie: *const Trie, filename: *const c_char);
    pub fn trie_destroy(trie: *const Trie);
    pub fn trie_mmap(trie: *const Trie, filename: *const c_char);
}
