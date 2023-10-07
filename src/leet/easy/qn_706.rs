// Design HashMap

pub struct MyHashMap {
    values: Vec<(i32, i32)>,
}

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap { values: vec![] }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.values.iter_mut().find(|val| val.0 == key) {
            val.1 = value;
        } else {
            self.values.push((key, value));
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        if let Some(val) = self.values.iter().find(|val| val.0 == key) {
            val.1
        } else {
            -1
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Some(position) = self.values.iter().position(|val| val.0 == key) {
            self.values.remove(position);
        }
    }
}

impl Default for MyHashMap {
    fn default() -> Self {
        MyHashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }
}
