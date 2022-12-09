pub mod day_00;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

#[allow(dead_code)]
pub enum SolverType {
    USize,
    ISize,
    String,
}

pub const DAYS: usize = 26;
pub const SOLS: [(*const (), SolverType); DAYS] = [
    (day_00::solve as *const (), SolverType::USize),
    (day_01::solve as *const (), SolverType::USize),
    (day_02::solve as *const (), SolverType::ISize),
    (day_03::solve as *const (), SolverType::USize),
    (day_04::solve as *const (), SolverType::USize),
    (day_05::solve as *const (), SolverType::USize),
    (day_06::solve as *const (), SolverType::USize),
    (day_07::solve as *const (), SolverType::USize),
    (day_08::solve as *const (), SolverType::USize),
    (day_09::solve as *const (), SolverType::USize),
    (day_10::solve as *const (), SolverType::USize),
    (day_11::solve as *const (), SolverType::USize),
    (day_12::solve as *const (), SolverType::USize),
    (day_13::solve as *const (), SolverType::USize),
    (day_14::solve as *const (), SolverType::USize),
    (day_15::solve as *const (), SolverType::USize),
    (day_16::solve as *const (), SolverType::USize),
    (day_17::solve as *const (), SolverType::USize),
    (day_18::solve as *const (), SolverType::USize),
    (day_19::solve as *const (), SolverType::USize),
    (day_20::solve as *const (), SolverType::USize),
    (day_21::solve as *const (), SolverType::USize),
    (day_22::solve as *const (), SolverType::ISize),
    (day_23::solve as *const (), SolverType::USize),
    (day_24::solve as *const (), SolverType::USize),
    (day_25::solve as *const (), SolverType::USize),
];

pub unsafe fn solve(contents: &str, day: usize) -> (String, String) {
    let (solver, solver_type) = &SOLS[day];

    let (part1, part2): (String, String) = unsafe {
        match solver_type {
            SolverType::USize => {
                let code = std::mem::transmute::<&*const (), &fn(&str) -> (usize, usize)>(solver);
                let (part1, part2) = code(contents);
                (part1.to_string(), part2.to_string())
            }
            SolverType::ISize => {
                let code = std::mem::transmute::<&*const (), &fn(&str) -> (isize, isize)>(solver);
                let (part1, part2) = code(contents);
                (part1.to_string(), part2.to_string())
            }
            SolverType::String => {
                let code = std::mem::transmute::<&*const (), &fn(&str) -> (String, String)>(solver);
                let (part1, part2) = code(contents);
                (part1, part2)
            }
        }
    };

    (part1, part2)
}
