use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut values_to_sort = Vec::new();

    for _i in 1..101{
        values_to_sort.push(rng.gen_range(0..1000));
    }

    bubble_sort( values_to_sort.clone());

}

// Non optimized bubble sort. 
fn bubble_sort(mut sorting_values: Vec<i32>){
    let mut swapped = true;

    while swapped == true{
        swapped = false;
        for i in 0.. sorting_values.len()-1{
            if sorting_values[i] > sorting_values[i+1]{
                sorting_values.swap(i, i+1);
                swapped = true
            }
        }
    }

    println!("{:?}", sorting_values);
}
