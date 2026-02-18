#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let es_sublista = |p: &[i32], g: &[i32]| p.is_empty() || g.windows(p.len()).any(|v| v == p);

    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (len_a, len_b) if len_a == len_b && first_list == second_list => Comparison::Equal,
        (len_a, len_b) if len_a < len_b && es_sublista(first_list, second_list) => {
            Comparison::Sublist
        }
        (len_a, len_b) if len_a > len_b && es_sublista(second_list, first_list) => {
            Comparison::Superlist
        }
        _ => Comparison::Unequal,
    }
}
