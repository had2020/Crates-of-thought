enum PipDir {TR,TL,BR,BL,T,B,L,R,C}
fn source_check(c:char, x: usize, y: usize, m_x: usize, m_y: usize) -> (bool, char) {
    let pipdir = match (x,y) {
        (0,0) => PipDir::TL,
        (0,m_y) => PipDir::TR, 
        (m_x,0) => PipDir::BL,
        (m_x,m_y) => PipDir::BR,
        _ => {},
        
    }
    let r = match (c, pipdir) {
        '┗' => (true, c),
        '┓' => (true, c),
        '┏' => (true, c),
        '┛' => (true, c),
        '━' => (true, c),
        '┃' => (true, c),
        '┣' => (true, c),
        '┫' => (true, c),
        '┳' => (true, c),
        '┻' => (true, c),
        '╋' => (true, c),
        _ => (false, c),
    }; return r
}
fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut rt: Vec<Vec<char>> = Vec::new();
    for i in 0..pipe_map.len() {
        rt.push(pipe_map[i].chars().collect());
    } // ws edge scanning
    let mut ws_poss: Vec<(usize, usize, char)> = Vec::new();
    for i in 0..rt.len() { // TL and TR
        let n0 = source_check(rt[0][i]);
        if n0.0 {
            ws_poss.push((0,i, n0.1));
        }
        let n1 = source_check(rt[i][0]);
        if n1.0 {
            ws_poss.push((i,0, n1.1));
        }
    }

    let mut r: bool = true;
    for ws in ws_poss {
        let n_cels = [rt[ws.0][(ws.1)+1], rt[ws.0][(ws.1).saturating_sub(1)], rt[(ws.0)+1][ws.1], rt[(ws.0).saturating_sub(1)][ws.1]];
        for c in n_cels {
            todo!("matching pipes, and recersive!")
        }
    }
    return r
}
