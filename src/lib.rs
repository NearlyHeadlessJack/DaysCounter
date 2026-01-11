#![allow(unused)]

mod calendar;

use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(PartialEq)]
enum Order {
    Asc,
    Desc,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "升序"),
            Order::Desc => write!(f, "降序"),
        }
    }
}

pub fn calculator(date1: (i32, i32, i32), date2: (i32, i32, i32)) -> i32 {
    println!(
        "计算程序开始，第一个日期:{:?}\n\
    第二个日期:{:?}",
        date1, date2
    );

    let order: Order = check_order(date1, date2);
    let year1: (i32, i32, i32);
    let year2: (i32, i32, i32);
    match order {
        Order::Asc => {
            year1 = date1;
            year2 = date2;
        }
        Order::Desc => {
            year1 = date2;
            year2 = date1;
        }
    }

    #[cfg(debug_assertions)]
    println!("{}", order);
    let mut days = calendar::main(year1, year2);
    if order == Order::Desc {
        days = -days;
    }
    println!("{}", days);
    return days;
}

fn check_order(
    (year1, month1, day1): (i32, i32, i32),
    (year2, month2, day2): (i32, i32, i32),
) -> Order {
    if year1 > year2 {
        Order::Desc
    } else if year1 < year2 {
        return Order::Asc;
    } else if month1 > month2 {
        return Order::Desc;
    } else if month1 < month2 {
        return Order::Asc;
    } else if day1 > day2 {
        return Order::Desc;
    } else if day1 < day2 {
        return Order::Asc;
    } else {
        return Order::Asc;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
