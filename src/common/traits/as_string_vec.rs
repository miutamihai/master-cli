pub trait AsStringVec {
    fn as_string_vec(&self) -> Vec<String>;
}

impl AsStringVec for &'static str {
    fn as_string_vec(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl AsStringVec for Vec<&'static str> {
    fn as_string_vec(&self) -> Vec<String> {
        self.iter().map(|value| String::from(*value)).collect()
    }
}

impl AsStringVec for String {
    fn as_string_vec(&self) -> Vec<String> {
        vec![self.clone()]
    }
}

impl AsStringVec for Vec<String> {
    fn as_string_vec(&self) -> Vec<String> {
        self.clone()
    }
}

impl AsStringVec for &String {
    fn as_string_vec(&self) -> Vec<String> {
        let clone = (*self).clone();

        vec![clone]
    }
}
