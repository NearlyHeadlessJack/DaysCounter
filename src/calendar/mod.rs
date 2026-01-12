fn create_calendar(is_leap_year: bool) -> [[u8; 31]; 12] {
    let regular_months: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_year_months: [usize; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 0-1月，10-11月，11-12月
    let mut m: [[u8; 31]; 12] = [[0; 31]; 12];

    let month = |leap, i: usize| -> usize {
        if leap {
            leap_year_months[i]
        } else {
            regular_months[i]
        }
    };
    for i in 0..12 {
        for j in 0..=31 {
            if j >= month(is_leap_year, i) {
                #[cfg(debug_assertions)]
                println!("[DEBUG]第{}月,传入{}天", i + 1, month(is_leap_year, i));

                break;
            }
            m[i][j] = 1;
        }
    }
    m
}

pub fn main(
    (mut start_year, mut start_month, mut start_day): (i32, i32, i32),
    (end_year, end_month, end_day): (i32, i32, i32),
) -> i32 {
    let mut total_days: i32 = 0;

    // 算法一，创建所有日期的总日历.
    let mut calendar: Vec<[[u8; 31]; 12]> = Vec::new();
    for year in start_year..=end_year {
        #[cfg(debug_assertions)]
        println!(
            "[DEBUG]正在创建日历，年份:{:?},是否闰年:{:?}",
            year,
            check_leap(year)
        );

        calendar.push(create_calendar(check_leap(year)));
    }
    let original_start_year = start_year;
    while !check_reach(
        (start_year, start_month, start_day),
        (end_year, end_month, end_day),
    ) {
        total_days += calendar[(start_year - original_start_year) as usize]
            [(start_month - 1) as usize][(start_day - 1) as usize] as i32;
        if start_month == 12 && start_day == 31 {
            start_month = 1;
            start_year += 1;
            start_day = 1;
        } else if start_day == 31 {
            start_month += 1;
            start_day = 1;
        } else {
            start_day += 1;
        }
    }

    total_days
}

fn check_reach(
    (year, month, day): (i32, i32, i32),
    (year2, month2, day2): (i32, i32, i32),
) -> bool {
    if year > year2 {
        return true;
    } else if year == year2 {
        if month > month2 {
            return true;
        } else if month == month2 {
            if day >= day2 {
                return true;
            }
        }
    }
    false
}
fn check_leap(year: i32) -> bool {
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    } else if year % 400 == 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_calendar() {
        let m = create_calendar(true);
        assert_eq!(m[1][28], 1);
        assert_eq!(m[1][27], 1);
        assert_eq!(m[0][30], 1);
        assert_eq!(m[11][30], 1);
        assert_eq!(m[2][30], 1);
        assert_eq!(m[3][30], 0);
        assert_eq!(m[3][29], 1);
        let m2 = create_calendar(false);
        assert_ne!(m2[1][28], 1);
        assert_eq!(m2[1][27], 1)
    }
}
