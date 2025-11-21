fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut rt: Vec<Vec<char>> = Vec::new();
    for i in 0..pipe_map.len() {
        rt.push(pipe_map[i].chars().collect());
    }
    let mut ws_poss: Vec<(usize, usize)> = Vec::new();
    for i in 0..rt.len() { // ws scanning
        for j in 0..rt[i].len() {
            if rt[i][j] == '+' {
                ws_poss.push((i,j));
            }
        }
    }
    let mut r: bool = true;
    for ws in ws_poss {
        let n_cels = [rt[ws.0][(ws.1)+1], rt[ws.0][(ws.1).saturating_sub(1)], rt[(ws.0)+1][ws.1], rt[(ws.0).saturating_sub(1)][ws.1]];
        for _ in n_cels
    }
    return r
}
