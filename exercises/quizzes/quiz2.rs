// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod my_module {
    use crate::Command; // modules root dung crate -> modules cha dung super

    // TODO: Complete the function as described above.
    pub(crate) fn transformer(input: Vec<(String , Command)>) -> Vec<String> {
        input
        .into_iter()
        .map(|(text , command )| match command {
            Command::Uppercase => text.to_uppercase(),
            Command::Trim => text.trim().to_string(),
            Command::Append(n) => text + &"bar".repeat(n),
        })
        .collect()
    }
    //Nhắc lại về phương thức vector
    //insert(index,value) : để thêm vào cuối mảng 
    //push() : thêm vào phần tử 
    //pop() lấy phần tử ra khỏi mảng 
    //remove(index)
    //get(index) : lấy ra Some()
    //iter() : Trả về itorator mượn &T
    //iter_mut() : Trả về itor chỉnh sửa được 
    //into_mut () : trả về itor Mượn T
    //sort() : sắp xếp vector
    //sort_by(|a,b| Tùy chỉnh )
    //reverse() đảo ngược 
    //map() + collect(): Chuyển đổi phần tử
    //constains(&value) -> bool : kiểm tra giá trị có không
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module;
    use super::Command;
    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = my_module::transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
