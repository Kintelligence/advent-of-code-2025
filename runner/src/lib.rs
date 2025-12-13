use shared::Solution;

pub fn day(index: u32) -> &'static (fn(&str) -> Solution, &'static str, &'static str) {
    match index {
        1 => &(day_01::part_1, day_01::_INPUT, "01.1"),
        2 => &(day_01::part_2, day_01::_INPUT, "01.2"),
        3 => &(day_02::part_1, day_02::_INPUT, "02.1"),
        4 => &(day_02::part_2, day_02::_INPUT, "02.2"),
        5 => &(day_03::part_1, day_03::_INPUT, "03.1"),
        6 => &(day_03::part_2, day_03::_INPUT, "03.2"),
        7 => &(day_04::part_1, day_04::_INPUT, "04.1"),
        8 => &(day_04::part_2, day_04::_INPUT, "04.2"),
        9 => &(day_05::part_1, day_05::_INPUT, "05.1"),
        10 => &(day_05::part_2, day_05::_INPUT, "05.2"),
        11 => &(day_06::part_1, day_06::_INPUT, "06.1"),
        12 => &(day_06::part_2, day_06::_INPUT, "06.2"),
        13 => &(day_07::part_1, day_07::_INPUT, "07.1"),
        14 => &(day_07::part_2, day_07::_INPUT, "07.2"),
        15 => &(day_08::part_1, day_08::_INPUT, "08.1"),
        16 => &(day_08::part_2, day_08::_INPUT, "08.2"),
        17 => &(day_09::part_1, day_09::_INPUT, "09.1"),
        18 => &(day_09::part_2, day_09::_INPUT, "09.2"),
        19 => &(day_10::part_1, day_10::_INPUT, "10.1"),
        20 => &(day_10::part_2, day_10::_INPUT, "10.2"),
        21 => &(day_11::part_1, day_11::_INPUT, "11.1"),
        22 => &(day_11::part_2, day_11::_INPUT, "11.2"),
        23 => &(day_12::part_1, day_12::_INPUT, "12.1"),
        _ => panic!("Index out of bounds"),
    }
}
