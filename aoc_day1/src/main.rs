fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let mut elves: Vec<i32> = Vec::with_capacity(250); 
    let mut elf_count = 0;
    let mut max_calories = 0;
    let mut second_calories: i32 = 0;
    let mut third_calories: i32 = 0;
    let mut pool_of_three: i32 = 0;

    for _i in 0..1000 {
        elves.push(0);
    }

    for line in input.lines() { 
        if line == "" {
            if elves[elf_count] > third_calories{
                if elves[elf_count] > second_calories{
                    if elves[elf_count] > max_calories{
                        if max_calories > second_calories{
                            second_calories = max_calories;
                        }
                        max_calories = elves[elf_count];
                    }
                    else {
                        if second_calories > third_calories{
                            third_calories = second_calories;
                        }
                        second_calories = elves[elf_count];
                    }
                }
                else { 
                    third_calories = elves[elf_count]; 
                }
            }
            elf_count = elf_count + 1;
        }
        else { 
            let snack = line.parse::<i32>().unwrap();
            // println!("{}", snack); 
            elves[elf_count] = elves[elf_count] + snack;
        }
        
    }
    println!("{}", max_calories);
    println!("{}", second_calories);
    println!("{}", third_calories);
    pool_of_three = max_calories + second_calories + third_calories;
    println!("The top three elves are carrying {} calories", pool_of_three);
    println!("The elf with the most calories has {} calories", max_calories);
    println!("There are {} elves", elf_count);

}