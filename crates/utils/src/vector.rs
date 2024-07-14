pub fn combine_positions_and_colors(positions: Vec<f32>, colors: Vec<f32>) -> Vec<f32> {
    positions
        .chunks(3)
        .zip(colors.chunks(3))
        .flat_map(|(pos, col)| pos.iter().chain(col.iter()))
        .copied()
        .collect()
}
