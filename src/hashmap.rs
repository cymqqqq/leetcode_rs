/*
Design HashMap
Design a HashMap without using any built-in hash table libraries.

To be specific, your design should include these functions:

put(key, value) : Insert a (key, value) pair into the HashMap. If the value already exists in the HashMap, update the value.
get(key): Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
remove(key) : Remove the mapping for the value key if this map contains the mapping for the key.

Example:

MyHashMap hashMap = new MyHashMap();
hashMap.put(1, 1);          
hashMap.put(2, 2);         
hashMap.get(1);            // returns 1
hashMap.get(3);            // returns -1 (not found)
hashMap.put(2, 1);          // update the existing value
hashMap.get(2);            // returns 1 
hashMap.remove(2);          // remove the mapping for 2
hashMap.get(2);            /
*/
#[derive(Default)]
struct MyHashmap {
    hm: Vec<i32>,
}
const LIMIT: usize = 1_000_000;
impl MyHashmap {
    fn new() -> Self {
        MyHashmap {
            hm: vec![-1; LIMIT + 1],
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        self.hm[key as usize] = value;
    }
    fn get(&self, key: i32) -> i32 {
        self.hm[key as usize]
    }
    fn remove(&mut self, key: i32) {
        self.hm[key as usize] = -1;
    }
}
