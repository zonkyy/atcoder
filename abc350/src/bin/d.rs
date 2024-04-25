use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("filed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    for (a, b) in ab.iter() {
        uf.unite(*a, *b);
    }

    let mut edge_cnt = HashMap::new();
    for (a, b) in ab.iter() {
        *edge_cnt.entry(uf.root(*a)).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in edge_cnt {
        let max_edge = uf.size(k) * (uf.size(k) - 1) / 2;
        ans += max_edge - v;
    }

    println!("{}", ans);
}
