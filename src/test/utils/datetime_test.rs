
#[cfg(test)]
mod test {
    use rust_wheel::common::util::time_util::end_of_today;

    #[test]
    fn end_of_day_test(){
        let end_of_day = end_of_today();
        println!("{}",end_of_day)
    }
}




