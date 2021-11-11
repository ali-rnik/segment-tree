macro_rules! parse_line {
    ($t: ty) => ({
        let mut a_str = String::new();
        std::io::stdin().read_line(&mut a_str).expect("read error");
        let a_str = a_str.trim();
        a_str.parse::<$t>().expect("parse error")
    });
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        std::io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
                a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

#[allow(unused_macros)]
macro_rules! parse_line_to_vec {
    ($t: ty) => {{
        let mut a_str = String::new();
        std::io::stdin().read_line(&mut a_str).expect("read error");
        (a_str
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>())
    }};
}

struct Segtree<'a> {
    nums: &'a Vec<i32>,
    st: Vec<i32>,
    lazy: Vec<i32>,
    qbegin: usize,
    qend: usize,
    new_val: i32,
}

impl Segtree<'_> {
    fn new(nums: &Vec<i32>) -> Segtree {
        let log = (nums.len() as f64).log2().ceil() as u32;
        let len = ((2 as i32).pow(log) * 2 - 1) as usize;
        let st = vec![std::i32::MAX; len];
        let lazy = vec![0; len];
        Segtree {
            nums,
            st,
            lazy,
            qbegin: 0,
            qend: 0,
            new_val: 0,
        }
    }

    fn print(&self) {
        println!("Segment Tree: {:?}", self.st);
        println!("Lazy Tree: {:?}", self.lazy);
    }

    fn modify(&mut self, qbegin: usize, qend: usize, new_val: i32) {
        if qend >= self.nums.len() {
            println!("err: please insert in bound range");
            return;
        }

        self.qbegin = qbegin;
        self.qend = qend;
        self.new_val = new_val;

        self.modify_runner(0, 0, self.nums.len() - 1);
    }

    fn modify_runner(&mut self, root: usize, begin: usize, end: usize) -> i32 {
        if self.lazy[root] != 0 {
            self.st[root] = self.lazy[root] * (end as i32 - begin as i32 + 1);
            if root * 2 + 2 < self.st.len() {
                self.lazy[root * 2 + 1] = self.lazy[root];
                self.lazy[root * 2 + 2] = self.lazy[root];
            }
            self.lazy[root] = 0;
        }

        if self.qbegin <= begin && self.qend >= end {
            self.st[root] = self.new_val * (end as i32 - begin as i32 + 1);
            if root * 2 + 2 < self.st.len() {
                self.lazy[root * 2 + 1] = self.new_val;
                self.lazy[root * 2 + 2] = self.new_val;
            }
            return self.st[root];
        } else if self.qend < begin || self.qbegin > end {
            return self.st[root];
        } else {
            let mid = (begin + end) / 2;
            let left = self.modify_runner(root * 2 + 1, begin, mid);
            let right = self.modify_runner(root * 2 + 2, mid + 1, end);

            self.st[root] = left + right;
            return self.st[root];
        }
    }

    fn create(&mut self) {
        self.create_runner(0, 0, self.nums.len() - 1);
    }

    fn create_runner(&mut self, root: usize, begin: usize, end: usize) -> i32 {
        if begin == end {
            self.st[root] = self.nums[begin];
            return self.st[root];
        }

        let mid = (begin + end) / 2;

        self.st[root] = self.create_runner(2 * root + 1, begin, mid)
            + self.create_runner(2 * root + 2, mid + 1, end);

        return self.st[root];
    }

    fn sum(&mut self, qbegin: usize, qend: usize) -> i32 {
        if qend >= self.nums.len() {
            println!("err: please insert in bound range");
            return -1;
        }

        self.qbegin = qbegin;
        self.qend = qend;

        return self.sum_runner(0, 0, self.nums.len() - 1);
    }

    fn sum_runner(&mut self, root: usize, begin: usize, end: usize) -> i32 {
        if self.lazy[root] != 0 {
            self.st[root] = self.lazy[root] * (begin as i32 - end as i32 + 1);
            if root * 2 + 2 < self.st.len() {
                self.lazy[root * 2 + 1] = self.lazy[root];
                self.lazy[root * 2 + 2] = self.lazy[root];
            }
            self.lazy[root] = 0;
        }

        if self.qbegin <= begin && self.qend >= end {
            return self.st[root];
        } else if self.qbegin > end || self.qend < begin {
            return 0;
        } else {
            let mid = (begin + end) / 2;
            let left = self.sum_runner(root * 2 + 1, begin, mid);
            let right = self.sum_runner(root * 2 + 2, mid + 1, end);
            return right + left;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_four_item() {
        let arr = vec![-1, 7, 1, 3];
        let mut st = Segtree::new(&arr);

        st.create();

        assert_eq!(vec![10, 6, 4, -1, 7, 1, 3], st.st);
    }

    #[test]
    fn create_three_item() {
        let arr = vec![-1, 7, 1];
        let mut st = Segtree::new(&arr);

        st.create();

        assert_eq!(vec![7, 6, 1, -1, 7, std::i32::MAX, std::i32::MAX], st.st);
    }

    #[test]
    fn create_two_item() {
        let arr = vec![-1, 7];
        let mut st = Segtree::new(&arr);

        st.create();

        assert_eq!(vec![6, -1, 7], st.st);
    }
    
    #[test]
    fn create_one_item() {
        let arr = vec![-1];
        let mut st = Segtree::new(&arr);

        st.create();

        assert_eq!(vec![-1], st.st);
    }
}
