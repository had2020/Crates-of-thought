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
    
    return false
}
