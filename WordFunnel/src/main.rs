use std::env::args;

fn main() 
{
    let mut params = args().skip(1).take(2);
    let mut base_word: String = params.next().unwrap();
    let sub_word: String = params.next().unwrap();
    for i in 0..sub_word.len()
    {
        if base_word.chars().nth(i) != sub_word.chars().nth(i)
        {
            base_word.remove(i);
            if base_word == sub_word
            {
                println!("They funnel");
                return;
            }
        }
    }
    println!("'{}' cannot be made into '{}'", base_word, sub_word);

    
}