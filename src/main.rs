mod functions;
use std::fs;

fn main() {
    let path_to_file = "ping_from_1_1_1_1.log";
    let string_from_file = fs::read_to_string(path_to_file).expect("Error");
    let c = string_from_file.split(';');
    let v : Vec<&str> = c.collect();
    let e = "4233772799:50.97";
    let b : Vec<&str> = e.split(':').collect();
    let t : u64 = b[0].parse().unwrap();
    let p : f32 = b[1].parse().unwrap();
    struct DataTimeRegular {
        year: u64, month: u64, day: u64,
        hour: u64, minute: u64, second: u64
    }
    let mut d = DataTimeRegular {
        year: 1970, month: 1, day: 1,
        hour: 0, minute: 0, second: 0
    };
    const NUMBER_OF_YEARS_IN_A_EPOCH:                   u64 = 4;
    const NUMBER_OF_SECONDS_IN_A_FOUR_YEAR_LEAP_CYCLE:  u64 = 126230400;
    const NUMBER_OF_SECONDS_IN_A_LEAP_YEAR:             u64 = 31622400;
    const NUMBER_OF_SECONDS_IN_A_YEAR:                  u64 = 31536000;
    const NUMBER_OF_SECONDS_IN_A_MONTH:                 u64 = 2629800;
    const NUMBER_OF_SECONDS_IN_A_DAY:                   u64 = 86400;
    const NUMBER_OF_SECONDS_IN_A_HOUR:                  u64 = 3600;
    const NUMBER_OF_SECONDS_IN_A_MINUTES:               u64 = 60;
    const NUMBER_OF_SECONDS_IN_A_SECONDS:               u64 = 1;
        //-----------------------------------------------------------------------------------//.
        // 8.2.2. Опис КОСТАНТ щодо кількості секунд у всіх місяцях--------------------------//.
            const NUMBER_OF_SECONDS_IN_A_JANUARY:               u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_FEBRUARY:              u64 = 2419200; //28
            const NUMBER_OF_SECONDS_IN_A_LEAP_FEBRUARY:         u64 = 2505600; //29
            const NUMBER_OF_SECONDS_IN_A_MARCH:                 u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_APRIL:                 u64 = 2592000; //30
            const NUMBER_OF_SECONDS_IN_A_MAY:                   u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_JUNE:                  u64 = 2592000; //30
            const NUMBER_OF_SECONDS_IN_A_JULY:                  u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_AUGUST:                u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_SEPTEMBER:             u64 = 2592000; //30
            const NUMBER_OF_SECONDS_IN_A_OCTOBER:               u64 = 2678400; //31
            const NUMBER_OF_SECONDS_IN_A_NOVEMBER:              u64 = 2592000; //30
            const NUMBER_OF_SECONDS_IN_A_DECEMBER:              u64 = 2678400; //31
        //-----------------------------------------------------------------------------------//.
        // 8.3. Вираховуємо кількість епох чотирирічного високосного циклу-------------------//.
            let num_epoch = t / NUMBER_OF_SECONDS_IN_A_FOUR_YEAR_LEAP_CYCLE;
            d.year = d.year + num_epoch * NUMBER_OF_YEARS_IN_A_EPOCH;
        // 8.4. Вираховуємо остачу секунд з якої будемо рахувати кількість років-------------//.
            let mut remain = t - num_epoch * NUMBER_OF_SECONDS_IN_A_FOUR_YEAR_LEAP_CYCLE;
        //-----------------------------------------------------------------------------------//.
        // 8.5. Визначаємо чи являється підзвітній рік високосним----------------------------//.
            let mut leap_year : bool = false;
            match remain {
                r if r > 0 && r < (2 * NUMBER_OF_SECONDS_IN_A_YEAR)            => leap_year = false,
                remain if remain > 63072000 && remain < 94694399     => leap_year = true,
                94694400..=126230400    => leap_year = false,
                _                       => println!("Error")
            }
        //-----------------------------------------------------------------------------------//.
        // 8.6. Визначаємо логікою яку функцію буде виконувати движок: невисокосну/високосну-//.
            match leap_year {
                false                   =>
                    //-----------------------Початок підфункції невисокосного року-----------------------//.
                                                        {
                    // 8.6.1. Вираховуємо рік та остачу секунд з якої будемо вираховувати місяці---------//.
                        let mut remain_for_months: u64 = 0;
                        match remain {
                            0..=31535999            => {remain_for_months = remain}
                            31536000..=63071999     => {d.year = d.year + 1; remain_for_months = remain - 31536000}
                            63072000..=94694399     => {d.year = d.year + 2; remain_for_months = remain - 63072000}
                            94694400..=126230399    => {d.year = d.year + 3; remain_for_months = remain - 94694400}
                            _        => println!("помилка роки")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.6.2. Вираховуємо місяць та остачу остачу секунд з якої будемо вираховувати дні----//.
                        let mut remain_for_days: u64 = 0;
                        match remain_for_months {
                            0..=2678399              => {                       remain_for_days = remain_for_months}
                            2678400..=5097599        => {d.month = d.month + 1; remain_for_days = remain_for_months - 2678400}
                            5097600..=7775999        => {d.month = d.month + 2; remain_for_days = remain_for_months - 5097600}
                            7776000..=10367999       => {d.month = d.month + 3; remain_for_days = remain_for_months - 7776000}
                            10368000..=13046399      => {d.month = d.month + 4; remain_for_days = remain_for_months - 10368000}
                            13046400..=15638399      => {d.month = d.month + 5; remain_for_days = remain_for_months - 13046400}
                            15638400..=18316799      => {d.month = d.month + 6; remain_for_days = remain_for_months - 15638400}
                            18316800..=20995199      => {d.month = d.month + 7; remain_for_days = remain_for_months - 18316800}
                            20995200..=23587199      => {d.month = d.month + 8; remain_for_days = remain_for_months - 20995200}
                            23587200..=26265599      => {d.month = d.month + 9; remain_for_days = remain_for_months - 23587200}
                            26265600..=28857599      => {d.month = d.month + 10; remain_for_days = remain_for_months - 26265600}
                            28857600..=31535999      => {d.month = d.month + 11; remain_for_days = remain_for_months - 28857600}
                            _                        => println!("помилка місяць")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.6.3. Вираховуємо дні та остачу секунд з якої будемо вираховувати години-----------//.
                        let mut remain_for_hours: u64 = 0;
                        match remain_for_days {
                            0           => {remain_for_hours = remain_for_days}
                            1..=2678399 => {d.day = d.day + remain_for_days / 86400; remain_for_hours = remain_for_days - (d.day-1) * 86400}
                            _           => println!("помилка дні")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.6.4. Вираховуємо години та остачу секунд з якої будемо вираховувати хвилини-------//.
                        let mut remain_for_minutes: u64 = 0;
                        match remain_for_hours {
                            0           => {remain_for_minutes = remain_for_hours}
                            1..=86399 => {d.hour = remain_for_hours / 3600; remain_for_minutes = remain_for_hours - d.hour * 3600}
                            _           => println!("помилка години")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.6.5. Вираховуємо хвилини та остачу секунд з якої будемо вираховувати секунди----//.
                        let mut remain_for_seconds: u64 = 0;
                        match remain_for_minutes {
                            0           => {remain_for_seconds = remain_for_hours}
                            1..=3599    => {d.minute = remain_for_minutes / 60;
                                           remain_for_seconds = remain_for_minutes - d.minute * 60;
                                           d.second = remain_for_seconds}
                            _           => println!("помилка хвилини")
                        }
                                                        }
                        //------------------------Кінець підфункції невисокосного року-------_---------------//.

                true                    =>
                        //-----------------------Початок підфункції високосного року-------------------------//.
                                                {
                        // 8.7.1. Вираховуємо рік та остачу секунд з якої будемо вираховувати місяці---------//.
                        let mut remain_for_months: u64 = 0;
                        match remain {
                            0..=31535999                =>                      {remain_for_months = remain}
                            31536000..=63071999         => {d.year = d.year + 1; remain_for_months = remain - 31536000}
                            63072000..=94694399         => {d.year = d.year + 2; remain_for_months = remain - 63072000}
                            94694400..=126230399        => {d.year = d.year + 3; remain_for_months = remain - 94694400}
                            _                           => println!("помилка року високосного")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.7.2. Вираховуємо місяць та остачу остачу секунд з якої будемо вираховувати дні--//.
                        let mut remain_for_days: u64 = 0;
                        match remain_for_months {
                            0                        => {                       remain_for_days = remain_for_months}
                            1..=2678399              => {                       remain_for_days = remain_for_months}
                            2678400..=5183999        => {d.month = d.month + 1; remain_for_days = remain_for_months - 2678400}
                            5184000..=7862399        => {d.month = d.month + 2; remain_for_days = remain_for_months - 5184000}
                            7862400..=10454399       => {d.month = d.month + 3; remain_for_days = remain_for_months - 7862400}
                            10454400..=13132799      => {d.month = d.month + 4; remain_for_days = remain_for_months - 10454400}
                            13132800..=15638399      => {d.month = d.month + 5; remain_for_days = remain_for_months - 13132800}
                            15638400..=18403199      => {d.month = d.month + 6; remain_for_days = remain_for_months - 15724800}
                            18403200..=21081599      => {d.month = d.month + 7; remain_for_days = remain_for_months - 18403200}
                            21081600..=23673599      => {d.month = d.month + 8; remain_for_days = remain_for_months - 21081600}
                            23673600..=26351999      => {d.month = d.month + 9; remain_for_days = remain_for_months - 23673600}
                            26352000..=28943999      => {d.month = d.month + 10; remain_for_days = remain_for_months - 26352000}
                            28944000..=31622399      => {d.month = d.month + 11; remain_for_days = remain_for_months - 28944000}
                            _                        => println!("помилка високосний місяць")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.7.3. Вираховуємо дні та остачу секунд з якої будемо вираховувати години---------//.
                        let mut remain_for_hours: u64 = 0;
                        match remain_for_days {
                            0           => {remain_for_hours = remain_for_days}
                            1..=2678399 => {d.day = d.day + remain_for_days / 86400; remain_for_hours = remain_for_days - (d.day-1) * 86400}
                            _           => println!("помилка дні")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.7.4. Вираховуємо години та остачу секунд з якої будемо вираховувати хвилини-----//.
                        let mut remain_for_minutes: u64 = 0;
                        match remain_for_hours {
                            0           => {remain_for_minutes = remain_for_hours}
                            1..=86399 => {d.hour = remain_for_hours / 3600; remain_for_minutes = remain_for_hours - d.hour * 3600}
                            _           => println!("помилка години")
                        }
                        //-----------------------------------------------------------------------------------//.
                        // 8.7.5. Вираховуємо хвилини та остачу секунд з якої будемо вираховувати секунди----//.
                        let mut remain_for_seconds: u64 = 0;
                        match remain_for_minutes {
                            0           => {remain_for_seconds = remain_for_hours}
                            1..=3599    => {d.minute = remain_for_minutes / 60;
                                remain_for_seconds = remain_for_minutes - d.minute * 60;
                                d.second = remain_for_seconds}
                            _           => println!("помилка високосні хвилини")
                        }
                    }
                //-------------------------Кінець підфункції високосного року--------------------//.
            }
    // 8.8. Вираховуємо день тижня-----------------------------------------------------------//.
        let mut num_all_day_of_a_unix_epoch = t / 86400;
        let num_all_a_week = num_all_day_of_a_unix_epoch / 7;
        let rem_day_of_a_week = num_all_day_of_a_unix_epoch - num_all_a_week * 7;
        let mut day_of_week= "";
        match rem_day_of_a_week {
            0 => day_of_week = "Thu",
            1 => day_of_week = "Fri",
            2 => day_of_week = "Sat",
            3 => day_of_week = "Sun",
            4 => day_of_week = "Mon",
            5 => day_of_week = "Tue",
            6 => day_of_week = "Wed",
            _ => println!("Error week")
        }
    //-------------------------------------------------------------------------------------------//.
    // 8.8. Преобразовуємо місяць----------------------------------------------------------------//.
    let mut d_month = "";
    match d.month {
        1 => d_month = "Jan",
        2 => d_month = "Fed",
        3 => d_month = "Mar",
        4 => d_month = "Apr",
        5 => d_month = "May",
        6 => d_month = "Jun",
        7 => d_month = "Jul",
        8 => d_month = "Aug",
        9 => d_month = "Sep",
        10 => d_month = "Oct",
        11 => d_month = "Nov",
        12 => d_month = "Dec",
        _ => println!("Error d_month")
    }
    //-------------------------------------------------------------------------------------------//.
    // 8.9. Виводимо результат функції-----------------------------------------------------------//.
        println!("--------------------------------------------------------");
        println!("|    Unix timestamp:    {}                     |", t);
        println!("|   Format RFC 2822:    {}, {} {} {} {}:{}:{} GMT   |", day_of_week, d.day, d_month, d.year, d.hour, d.minute, d.second);
        println!("--------------------------------------------------------");
    //-------------------------------------------------------------------------------------------//.
}