use chrono::NaiveDate;
use days_counter::calculator;
use rand::Rng;
#[test]
fn test_calculator() {
    let mut rng = rand::rng();
    const MAX_YEAR: i32 = 9999;
    for i in 1..=1000 {
        println!("\n\nterm: {}", i);
        let month1 = rng.random_range(1..=12);
        let day1 = rng.random_range(1..=28);
        let year1 = rng.random_range(1900..=MAX_YEAR);
        let month2 = rng.random_range(1..=12);
        let day2 = rng.random_range(1..=28);
        let year2 = rng.random_range(1900..=MAX_YEAR);
        assert_eq!(
            chrono_days((year1, month1, day1), (year2, month2, day2)),
            calculator(
                (year1, month1 as i32, day1 as i32),
                (year2, month2 as i32, day2 as i32)
            )
        );
    }
}

fn chrono_days((y1, m1, d1): (i32, u32, u32), (y2, m2, d2): (i32, u32, u32)) -> i32 {
    // 使用 from_ymd_opt 创建日期（推荐方式）
    let start = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
    let end = NaiveDate::from_ymd_opt(y2, m2, d2).unwrap();

    // 计算天数差
    let days = end.signed_duration_since(start).num_days();
    days as i32
}
