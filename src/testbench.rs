use crate::utils::{into_v16, from_v16};

pub fn get_xxx_end_index(cases: Vec<(&str, Option<usize>)>, func: fn(&[u16], usize) -> Option<usize>){

    let mut cases = cases.into_iter().map(
        |(case, answer)|
        (0..4).map(
            |n|
            (
                vec![vec![' ' as u16; n], into_v16(case)].concat(),
                n,
                match answer {
                    None => None,
                    Some(ans) => Some(ans + n)
                }
            )
        ).collect()
    ).collect::<Vec<Vec<(Vec<u16>, usize, Option<usize>)>>>().concat();

    // xxx cannot be empty
    cases.push((vec![], 0, None));
    cases.push((vec![], 1, None));
    cases.push((vec![], 2, None));

    let mut errors = vec![];

    for (case, start, answer) in cases.iter() {
        let result = func(case, *start);

        if &result != answer {
            errors.push(
                format!(
                    "case: `{}`\nstart: {}\nanswer: {:?}\nactual result: {:?}",
                    from_v16(case), start, answer, result
                )
            );
        }

    }

    if errors.len() > 0 {
        panic!("{} out of {} tests have failed!\n\n{}{}", errors.len(), cases.len(), errors.join("\n\n"), "\n\n------------------");
    }

}