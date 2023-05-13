use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_can_greet() {
        let result = greet("Bob");
        assert_eq!(result, "Hello, Bob!");
    }
}
