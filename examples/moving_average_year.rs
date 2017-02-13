fn compute_moving_average(window: &[(u32, u32)]) -> (u32, f32) {
    let window_size = window.len();
    println!("window.len(): {}", window.len());
    println!("window_size: {}", window_size);

    let current_year = window[window_size / 2].0;
    println!("current_year: {}", current_year);
    // Summiert alle Werte im Window
    let sum: u32 = window.iter().map(|&(_, val)| val).sum();
    // Bildet den Mittelwert und macht ein Float daraus
    let sum = sum as f32 / window_size as f32;
    println!("temp moving average (current_year, sum): {:?}", (current_year, sum));
    (current_year, sum)
}

fn extract_moving_average_for_year(year: u32, moving_average: &[(u32, f32)]) -> Option<f32> {
    moving_average
        .iter()
        .find(|&&(yr, _)| yr == year)
        .map(|&(_, val)| val)
}


fn main() {
    let vec = [
    (2003, 4),
    (2004, 6),
    (2005, 5),
    (2006, 8),
    (2007, 9),
    (2008, 5),
    (2009, 4),
    (2010, 3),
    (2011, 7),
    (2012, 9),
    (2013, 4),
    ];

    let moving_average = vec
        .windows(5)
        .map(compute_moving_average)
        .collect::<Vec<_>>();

    let moving_average = vec
        .iter()
        .map(|&(year, val)| (year, val, extract_moving_average_for_year(year, &moving_average)));

    for a in moving_average {
        println!("{:?}", a);
    }
}
