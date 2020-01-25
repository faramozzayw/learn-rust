pub trait Restrict {}

impl Restrict for i32 {}
impl Restrict for f32 {}
impl Restrict for f64 {}

pub fn find_max<T>(arr: &Vec<T>) -> T
where T: Restrict + PartialEq + PartialOrd + Clone
{
    let mut max = arr[0].clone();
    for item in arr.iter() {
        if item > &max {
            max = item.clone()
        }
    }

    max
}