use clap::{Arg, Command};
use days_counter::calculator;
fn main() {
    let args = Command::new("days-counter")
        .about("测试程序")
        .after_help(
            "e.g.\n   days_counter -s 2026-01-10 -e 2027-03-05\n\
        or days_counter -s 20260110 -e 20270305",
        )
        .arg(
            Arg::new("start_date")
                .short('s')
                .long("start")
                .value_name("DATE_START")
                .required(true)
                .help("输入开始日期"),
        )
        .arg(
            Arg::new("end_date")
                .short('e')
                .long("end")
                .value_name("DATE_END")
                .required(true)
                .help("输入结束日期"),
        )
        .get_matches();

    run(args);
}
fn run(args: clap::ArgMatches) {
    let data_s: String;
    let data_e: String;
    if let Some(date1) = args.get_one::<String>("start_date") {
        #[cfg(debug_assertions)]
        println!("start date: {}", date1);
        data_s = date1.to_string();
    } else {
        println!(
            "请输入开始日期！\
         2026-01-01 或 20260101 格式"
        );
        return;
    }

    if let Some(date2) = args.get_one::<String>("end_date") {
        #[cfg(debug_assertions)]
        println!("end date: {}", date2);
        data_e = date2.to_string();
    } else {
        println!(
            "请输入开始日期！\
         2026-01-03 或 20260103 格式"
        );
        return;
    }

    let date_start: (i32, i32, i32) =
        check_input(&data_s).expect("请输入正确的开始日期格式！\ne.g. 2026-01-03 或 20260103");
    let date_end: (i32, i32, i32) =
        check_input(&data_e).expect("请输入正确的结束日期格式！\ne.g. 2026-01-03 或 20260103");
    if !leap_day_check(date_start) || !leap_day_check(date_end) {
        println!("输入的日期有误！请重新检查闰月。");
        return;
    }
    calculator(date_start, date_end);
}

fn check_input(s: &str) -> Option<(i32, i32, i32)> {
    let year: i32;
    let month: i32;
    let day: i32;
    if s.len() == 8 {
        // 将 20260110 解析
        let _year = format!("{}", &s[0..4]);
        year =
            date_check(&_year, DateType::Year).expect("年份输入不正确，请输入1900至9999之间的年份");
        let _month = format!("{}", &s[4..6]);
        month = date_check(&_month, DateType::Month).expect("月份输入不正确");
        let _day = format!("{}", &s[6..8]);
        day = date_check(&_day, DateType::Day).expect("日输入不正确");

        return Some((year, month, day));
    } else if s.contains("-") && s.len() == 10 {
        // 将 2026-01-10 解析
        let _year = format!("{}", &s[0..4]);
        year =
            date_check(&_year, DateType::Year).expect("年份输入不正确，请输入1900至9999之间的年份");
        let _month = format!("{}", &s[5..7]);
        month = date_check(&_month, DateType::Month).expect("月份输入不正确");
        let _day = format!("{}", &s[8..10]);
        day = date_check(&_day, DateType::Day).expect("日输入不正确");

        return Some((year, month, day));
    }
    None
}

#[derive(PartialEq)]
enum DateType {
    Year,
    Month,
    Day,
}
fn date_check(s: &str, date_type: DateType) -> Option<i32> {
    let s: i32 = s.parse::<i32>().expect("输入不合法，parser error!");
    if date_type == DateType::Year {
        return if s >= 1900 && s <= 9999 {
            Some(s)
        } else {
            None
        };
    };

    if date_type == DateType::Month {
        return if s >= 1 && s <= 12 { Some(s) } else { None };
    };

    if date_type == DateType::Day {
        return if s >= 1 && s <= 31 { Some(s) } else { None };
    };
    None
}

fn leap_day_check((year, month, day): (i32, i32, i32)) -> bool {
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        if month == 2 {
            return day <= 29;
        }
    } else {
        if month == 2 {
            return day <= 28;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_input() {
        let args = make_input("2026-01-01", "2026-01-03");
        run(args);
    }
    fn make_input(date_s: &'static str, date_e: &'static str) -> clap::ArgMatches {
        let args = Command::new("days-counter")
            .about("测试程序")
            .arg(
                Arg::new("start_date")
                    .short('s')
                    .long("start")
                    .value_name("DATE_START")
                    // .required(true)
                    .default_value(date_s)
                    .help("输入开始日期"),
            )
            .arg(
                Arg::new("end_date")
                    .short('e')
                    .long("end")
                    .value_name("DATE_END")
                    // .required(true)
                    .default_value(date_e)
                    .help("输入结束日期"),
            )
            .get_matches();
        args
    }
}
