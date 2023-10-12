// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut s = String::from("");
    let mut flag = 0;
    for i in input.chars() {
        if i ==' '&& flag == 0{
            continue;
        }else {
            if flag == 0{
                flag = 1;
            }
            if i == '!'{
                flag = 0;
            }
            s.push(i);
        }
    }
    s
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s = input.to_string();
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut s = String::from("");
    let mut temp = String::from("");
    for i in input.chars() {
        if i !=' '{
            temp.push(i);
        }else{
            if temp == "cars".to_string(){
                s.push_str("balloons ");
                temp.clear();
            }else{
                temp.push(' ');
                s.push_str(&temp[..]);
                temp.clear();
            }
        }
    }
    if temp == "cars".to_string(){
        s.push_str("balloons");
    }else{
        s.push_str(&temp[..]);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
