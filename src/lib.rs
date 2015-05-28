extern crate libc;
use libc::{c_char, c_int, size_t};

#[repr(C)] pub struct KeySet;
#[repr(C)] pub struct Trie;
#[repr(C)] pub struct Key;
#[repr(C)] pub struct Agent;

extern "C" {
    pub fn keyset_create() -> *const KeySet;
    pub fn keyset_push(keyset: *const KeySet, string: *const c_char, len: c_int, val: c_int);
    pub fn keyset_num_keys(keyset: *const KeySet) -> size_t;
    pub fn keyset_destroy(keyset: *const KeySet);
    pub fn trie_create() -> *const Trie;
    pub fn trie_build(trie: *const Trie, keyset: *const KeySet, options: c_int);
    pub fn trie_save(trie: *const Trie, filename: *const c_char);
    pub fn trie_destroy(trie: *const Trie);
    pub fn trie_mmap(trie: *const Trie, filename: *const c_char);
    pub fn agent_create() -> *const Agent;
    pub fn agent_destroy(agent: *const Agent);
    pub fn set_query(agent: *const Agent, string: *const c_char);
    pub fn get_key(agent: *const Agent);
    pub fn get_value(key: *const Key) -> *const c_char;
    pub fn predictive_search(trie: *const Trie, agent: *const Agent) -> c_char; // bool
    pub fn key_destroy(key: *const Key);
}
