use std::collections::HashMap;

fn parse_number_string(s: &str) -> Result<u64, &'static str>
{
    let units: HashMap<&str, u64> =
        [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)].iter().cloned().collect();

    let teens: HashMap<&str, u64> =
        [("eleven", 11), ("twelve", 12), ("thirteen", 13), ("fourteen", 14), ("fifteen", 15), ("sixteen", 16), ("seventeen", 17), ("eighteen", 18), ("nineteen", 19)]
            .iter()
            .cloned()
            .collect();

    let tens: HashMap<&str, u64> =
        [("ten", 10), ("twenty", 20), ("thirty", 30), ("forty", 40), ("fifty", 50), ("sixty", 60), ("seventy", 70), ("eighty", 80), ("ninety", 90)]
            .iter()
            .cloned()
            .collect();

    let multiples: HashMap<&str, u64> = [("hundred", 100), ("thousand", 1_000), ("million", 1_000_000), ("billion", 1_000_000_000)].iter().cloned().collect();

    let mut result = 0u64;
    let mut current = 0u64;
    let words: Vec<&str> = s.split_whitespace().collect();

    for word in words
    {
        if let Some(&value) = units.get(word)
        {
            current += value;
        }
        else if let Some(&value) = teens.get(word)
        {
            current += value;
        }
        else if let Some(&value) = tens.get(word)
        {
            current += value;
        }
        else if let Some(&value) = multiples.get(word)
        {
            if value == 100
            {
                current *= value;
            }
            else
            {
                result += current * value;
                current = 0;
            }
        }
        else
        {
            return Err("Invalid word in number string");
        }
    }

    Ok(result + current)
}

fn main()
{
    match parse_number_string("six hundred ninety six billion nine hundred sixty nine million six hundred ninety six thousand nine hundred sixty nine")
    {
        Ok(value) => println!("Parsed value: {}", value),
        Err(e) => println!("Error: {}", e)
    }
}
