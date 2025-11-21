enum PipDir {TR,TL,BR,BL,T,B,L,R,C}
fn is_source(c:char, x: usize, y: usize, m_x: usize, m_y: usize) -> (bool, char) {
    let pipdir = match (x,y) {
        (0,0) => PipDir::TL,
        (m_x,0) => PipDir::TR, 
        (0,m_y) => PipDir::BL,
        (m_x,m_y) => PipDir::BR,
        (0,_) => {PipDir::T},
        (_,m_y) => PipDir::B,
        (_,0) => PipDir::L,
        (_,m_x) => PipDir::R,
        _ => PipDir::C,
    };
    let r = match pipdir {
        PipDir::TL => match c {
            '┛' => (false, c),
            _ => (true, c),
        },
        PipDir::TR => match c {
            '┗' => (false, c),
            _ => (true, c),
        },
        PipDir::BL => match c {
            '┓' => (false, c),
            _ => (true, c),
        },
        PipDir::BR => match c {
            '┏' => (false, c),
            _ => (true, c),
        },
        PipDir::T => match c {
            '━' => (false, c),
            _ => (true, c),
        },
        PipDir::B => match c {
            '━' => (false, c),
            _ => (true, c),
        },
        PipDir::L => match c {
            '┓' => (true, c),
            '┛' => (true, c),
            '━' => (true, c),
            '┫' => (true, c),
            '┳' => (true, c),
            '┻' => (true, c),
            '╋' => (true, c),
            _ => (true, c),
        },
        PipDir::R => match c {
            '┗' => (true, c),
            '┏' => (true, c),
            '━' => (true, c),
            '┣' => (true, c),
            '┳' => (true, c),
            '┻' => (true, c),
            '╋' => (true, c),
            _ => (true, c),
        },
        _ => (false, c)
    }; return r
}
fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut rt: Vec<Vec<char>> = Vec::new();
    for i in 0..pipe_map.len() {
        rt.push(pipe_map[i].chars().collect());
    } // ws edge scanning
    let mut ws_poss: Vec<(usize, usize, char)> = Vec::new();
    for i in 0..rt.len() { // TL and TR
        let n0 = is_source(rt[0][i], 0, i, rt[i].len()-1, rt.len()-1);
        if n0.0 {
            ws_poss.push((0,i, n0.1));
        }
        let n1 = is_source(rt[i][0], 0, i, rt[i].len()-1, rt.len()-1);
        if n1.0 {
            ws_poss.push((i,0, n1.1));
        }
    }
    let mut r: bool = true;
    for ws in ws_poss { 
        let t_cel = rt[ws.0][(ws.1)+1];
        let b_cel = rt[ws.0][(ws.1).saturating_sub(1)];
        let l_cel = rt[(ws.0)+1][ws.1];
        let r_cel = rt[(ws.0).saturating_sub(1)][ws.1];
        let mut vists: Vec<Vec<>>
        while 
    }
    return r
}
