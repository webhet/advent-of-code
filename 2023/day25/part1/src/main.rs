use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening input file: {err}");
            return;
        }
    };

    let reader = BufReader::new(file);

    let edges = parse_edges(reader);

    let (a_size, b_size) = kernighan_lin(edges);

    println!("Groups ({a_size}, {b_size}): {}", a_size * b_size);
}

fn kernighan_lin(edges: HashSet<(usize, usize)>) -> (usize, usize) {
    let vertices = edges
        .iter()
        .map(|(v, _)| *v)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let middle = vertices.len() / 2;
    let mut set_a = vertices
        .iter()
        .take(middle)
        .copied()
        .collect::<HashSet<_>>();
    let mut set_b = vertices
        .iter()
        .skip(middle)
        .copied()
        .collect::<HashSet<_>>();

    loop {
        // compute D values for all a in A and b in B
        let mut d_values = compute_d_values(&set_a, &set_b, &HashSet::new(), &edges);

        // let gv, av, and bv be empty lists
        let mut gv = Vec::new();
        let mut av = Vec::new();
        let mut bv = Vec::new();

        let mut seen = HashSet::new();

        // for n := 1 to |V| / 2 do
        for _ in 1..=middle {
            // find a from A and b from B, such that g = D[a] + D[b] − 2×c(a, b) is maximal
            let (a, b, g) = find_max_a_b_g_by_d_values(&set_a, &set_b, &seen, &d_values, &edges);

            // remove a and b from further consideration in this pass
            seen.insert(a);
            seen.insert(b);

            // add g to gv, a to av, and b to bv
            gv.push(g);
            av.push(a);
            bv.push(b);

            // update D values for the elements of A = A \ a and B = B \ b
            d_values = compute_d_values(&set_a, &set_b, &HashSet::from_iter([a, b]), &edges);
        }

        // find k which maximizes g_max, the sum of gv[1], ..., gv[k]
        let (g_max, k) = maximize_sum(&gv);

        println!("{g_max}");

        if g_max > 0 {
            // Exchange av[1], av[2], ..., av[k] with bv[1], bv[2], ..., bv[k]
            for i in 0..k {
                set_a.remove(&av[i]);
                set_b.remove(&bv[i]);
                set_a.insert(bv[i]);
                set_b.insert(av[i]);
            }
        } else {
            break;
        }
    }

    (set_a.len(), set_b.len())
}

fn compute_d_values(
    set_a: &HashSet<usize>,
    set_b: &HashSet<usize>,
    exclude: &HashSet<usize>,
    edges: &HashSet<(usize, usize)>,
) -> HashMap<usize, isize> {
    let mut map = HashMap::new();

    for a in set_a.difference(exclude) {
        let internal_cost = edges
            .iter()
            .filter(|(v, t)| v == a && set_a.contains(t))
            .count();
        let external_cost = edges
            .iter()
            .filter(|(v, t)| v == a && set_b.contains(t))
            .count();

        map.insert(*a, external_cost as isize - internal_cost as isize);
    }

    for b in set_b.difference(exclude) {
        let internal_cost = edges
            .iter()
            .filter(|(v, t)| v == b && set_b.contains(t))
            .count();
        let external_cost = edges
            .iter()
            .filter(|(v, t)| v == b && set_a.contains(t))
            .count();

        map.insert(*b, external_cost as isize - internal_cost as isize);
    }

    map
}

fn find_max_a_b_g_by_d_values(
    set_a: &HashSet<usize>,
    set_b: &HashSet<usize>,
    exclude: &HashSet<usize>,
    d_values: &HashMap<usize, isize>,
    edges: &HashSet<(usize, usize)>,
) -> (usize, usize, isize) {
    let mut a_max = 0;
    let mut b_max = 0;
    let mut g_max = isize::MIN;

    for a in set_a.difference(exclude) {
        for b in set_b.difference(exclude) {
            let edge_cost = if edges.contains(&(*a, *b)) { 1 } else { 0 };

            let g = *d_values.get(a).unwrap() + *d_values.get(b).unwrap() - 2 * edge_cost;

            if g > g_max {
                a_max = *a;
                b_max = *b;
                g_max = g;
            }
        }
    }

    (a_max, b_max, g_max)
}

fn maximize_sum(gv: &[isize]) -> (isize, usize) {
    let mut g_max = gv[0];
    let mut k = 0;

    for i in 2..=gv.len() {
        let sum = gv.iter().take(i).sum();

        if sum > g_max {
            g_max = sum;
            k = i - 1;
        }
    }

    (g_max, k)
}

fn parse_edges(reader: BufReader<File>) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let (lpart, rpart) = line.split_once(": ").expect("Line split failed");
        let right_parts = rpart.split_ascii_whitespace();

        let lnum = usize::from_str_radix(lpart, 36).expect("Num parse failed");

        for rp in right_parts {
            let rnum = usize::from_str_radix(rp, 36).expect("Num parse failed");

            set.insert((lnum, rnum));
            set.insert((rnum, lnum));
        }
    }

    set
}
