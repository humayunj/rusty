use std::collections::HashMap;

pub fn computeMode(v: &Vec<i32>) -> &i32 {
    let mut map: HashMap<&i32, usize> = HashMap::new();
    for i in v.iter() {
        let x = map.entry(i).or_insert(0);
        *x += 1;
    }
    let mut mode = v.get(0).unwrap_or(&0);
    let mut total = 0;

    for (k, v) in map {
        if v > total {
            mode = k;
            total = v;
        }
    }

    return mode;
}
