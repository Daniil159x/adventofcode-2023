pub fn solve(content: &str) -> u64 {
    let wight = content.find(|c| c == '\n').unwrap();

    let factor = 1_000_000;

    let galaxies = content
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(column, _)| (row, column))
        })
        .flatten()
        .collect::<Vec<_>>();

    let empty_columns = (0..wight)
        .map(|column| {
            content
                .lines()
                .all(|line| line.bytes().skip(column).next().unwrap() as char == '.')
        })
        .collect::<Vec<_>>();

    let empty_rows = content
        .lines()
        .map(|line| line.chars().all(|c| c == '.'))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let lhs = galaxies[i];
            let rhs = galaxies[j];

            // rows
            let r_start = lhs.0.min(rhs.0);
            let r_end = lhs.0.max(rhs.0);

            let r_extend =
                (r_start + 1..r_end).filter(|row| empty_rows[*row]).count() * (factor - 1);

            // columns
            let c_start = lhs.1.min(rhs.1);
            let c_end = lhs.1.max(rhs.1);

            let c_extend = (c_start + 1..c_end)
                .filter(|column| empty_columns[*column])
                .count()
                * (factor - 1);

            let distance = r_extend + c_extend + c_end - c_start + r_end - r_start;
            sum += distance;
        }
    }

    sum as u64
}
