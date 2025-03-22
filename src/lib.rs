// NOTE: If you use Nightly Rust then you can do magic.
// #![feature(const_cmp)]
// #![feature(const_trait_impl)]
// #![feature(derive_const)]

mod dim;
pub mod lexer;
pub mod parser;
mod unit;
pub mod units;

// Каждое поле представляет собой степень одной из 7 основных физических величин,
// определенных в Международной системе единиц (СИ), и вместе они составляют размерность.
// Все остальные физические величины считаются составными и образуются путем умножения и деления основных.
// Тип i8 представляет значения от -128 до 127, чего более чем достаточно для практических расчетов.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIDimension {
    l: i8, // length
    m: i8, // mass
    t: i8, // time
    i: i8, // electric current
    o: i8, // temperature
    n: i8, // amount of substance
    j: i8, // luminous intensity
}

// Может добавить такое?
// const PASCAL_UNIT: SIDimension = SIDimension::new()

// Стоит отметить что разделения на микро-, милли-, деци-, кило- и другие кратные величины *нет*,
// а внесистемные единицы должны быть преобразованы перед хранением.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIUnit {
    // NOTE: maybe f64 => bigdecimal/BigRational?
    value: f64,
    // SIDimension вынесен в отдельную структуру чтобы можно было `#[derive(PartialEq)]`
    // для проверки равенства размерностей во время сложения/вычитания
    dim: SIDimension,
}

// TODO: rename
#[derive(Debug)]
pub struct Magic {
    identifier: String,
    unit: SIUnit,
}

// struct Polynomial {
//     terms: Vec<SIUnit>,
// }

// TODO: особый вывод (print) для составных величин
// TODO: локализация для вывода
// TODO: tests
// TODO: векторные величины
