pub fn getColor(color: u32) -> String {
    // this method returns the color an edge can have or black if it is above the limit
    let colors = [
        "#01a120",
        "#ff106f",
        "#ff6f00",
        "#ffdf00",
        "#7fd0ff",
        "#3d7ddd",
        "#9fff0d",
        "#5d0f0f",
        "#601df0",
        "#3f3f00",
        "#ff3f3f",
        "#3fffff",
        "#ff3fff",
        "#3f117f",
        "#104a4f",
        "#3f3f3f",
        "#3f3fff",
    ];
    if color < 0 {
        return "#000000".to_owned();
    }
    if (color as usize) >= colors.len() {
        return "#000000".to_owned();
    }
    return colors[color as usize].to_owned();
}