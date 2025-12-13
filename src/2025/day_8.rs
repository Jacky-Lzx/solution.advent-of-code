use std::{collections::HashMap, fs::read_to_string};

/// 并查集（Union-Find/DSU）实现
#[derive(Debug, Clone)]
pub struct UnionSet<T: Eq + std::hash::Hash + Clone> {
    // 存储元素 -> 父节点的映射
    parent: HashMap<T, T>,
    // 存储根节点 -> 集合大小（用于按秩合并）
    size: HashMap<T, usize>,
}

impl<T: Eq + std::hash::Hash + Clone> Default for UnionSet<T> {
    fn default() -> Self {
        UnionSet {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }
}

impl<T: Eq + std::hash::Hash + Clone> UnionSet<T> {
    /// 创建空的并查集
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加元素到并查集（若不存在则创建独立集合）
    pub fn add(&mut self, x: T) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x.clone(), x.clone()); // 初始父节点是自身
            self.size.insert(x, 1); // 初始集合大小为1
        }
    }

    /// 查找元素的根节点（带路径压缩）
    /// 返回值：(是否找到根节点, 根节点)
    pub fn find(&mut self, x: T) -> Option<T> {
        if !self.parent.contains_key(&x) {
            return None; // 元素不存在
        }

        // 递归 + 路径压缩：将当前节点直接指向根节点
        let parent_x = self.parent.get(&x).unwrap().clone();
        if parent_x != x {
            let root = self.find(parent_x).unwrap().clone();
            self.parent.insert(x.clone(), root.clone()); // 路径压缩
            Some(root)
        } else {
            Some(x)
        }
    }

    /// 合并两个元素所属的集合
    /// 返回值：是否成功合并（元素不存在/已在同一集合则返回false）
    pub fn merge(&mut self, x: T, y: T) -> bool {
        // 确保两个元素都存在（不存在则添加）
        self.add(x.clone());
        self.add(y.clone());

        // 查找各自的根节点
        let root_x = self.find(x).unwrap().clone();
        let root_y = self.find(y).unwrap().clone();

        // 同一集合，无需合并
        if root_x == root_y {
            return false;
        }

        // 按秩合并：小集合合并到大集合下
        let size_x = *self.size.get(&root_x).unwrap();
        let size_y = *self.size.get(&root_y).unwrap();

        if size_x > size_y {
            // y的根指向x的根
            self.parent.insert(root_y.clone(), root_x.clone());
            // 更新大集合的大小
            self.size.insert(root_x, size_x + size_y);
        } else {
            // x的根指向y的根
            self.parent.insert(root_x.clone(), root_y.clone());
            self.size.insert(root_y, size_x + size_y);
        }

        true
    }

    /// 检查两个元素是否在同一集合
    pub fn is_connected(&mut self, x: T, y: T) -> bool {
        match (self.find(x), self.find(y)) {
            (Some(root_x), Some(root_y)) => root_x == root_y,
            _ => false, // 任一元素不存在则不连通
        }
    }

    /// 获取并查集中的元素总数
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// 检查并查集是否为空
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Location {
    x: i128,
    y: i128,
    z: i128,
}

struct Edge {
    distance: f64,
    box_1: Location,
    box_2: Location,
}

fn part_1(boxes: &[Location], connection_num: usize) {
    // Generate edge vectors

    let mut edges = vec![];

    for (i, box_i) in boxes.iter().enumerate() {
        boxes.iter().skip(i + 1).for_each(|other_box| {
            let distance = (((box_i.x - other_box.x).pow(2)
                + (box_i.y - other_box.y).pow(2)
                + (box_i.z - other_box.z).pow(2)) as f64)
                .sqrt();
            edges.push(Edge {
                distance,
                box_1: box_i.clone(),
                box_2: other_box.clone(),
            });
        });
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // edges
    //     .iter()
    //     .for_each(|x| println!("Edge distance: {}", x.distance));

    let mut bin: UnionSet<Location> = UnionSet::new();
    for b in boxes {
        bin.add(b.clone());
    }

    for edge in edges[0..connection_num].iter() {
        bin.merge(edge.box_1.clone(), edge.box_2.clone());
    }

    let mut map: HashMap<Location, i32> = HashMap::new();

    for b in boxes {
        let root = bin.find(b.clone()).unwrap();
        let count = map.entry(root).or_insert(0);
        *count += 1;
    }

    // Get the largest three values in the map
    let mut counts = map.values().cloned().collect::<Vec<i32>>();
    counts.sort_by(|a, b| b.cmp(a));
    let largest_three = &counts[..3.min(counts.len())];
    let result: i32 = largest_three.iter().product();

    println!("{:?}", counts);

    println!("Part 1: {}", result);
}

fn part_2(boxes: &[Location]) {
    // Generate edge vectors

    let mut edges = vec![];

    for (i, box_i) in boxes.iter().enumerate() {
        boxes.iter().skip(i + 1).for_each(|other_box| {
            let distance = (((box_i.x - other_box.x).pow(2)
                + (box_i.y - other_box.y).pow(2)
                + (box_i.z - other_box.z).pow(2)) as f64)
                .sqrt();
            edges.push(Edge {
                distance,
                box_1: box_i.clone(),
                box_2: other_box.clone(),
            });
        });
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut bin: UnionSet<Location> = UnionSet::new();
    for b in boxes {
        bin.add(b.clone());
    }

    let mut last_edge: Option<&Edge> = None;

    // for edge in edges[0..connection_num].iter() {
    //     bin.merge(edge.box_1.clone(), edge.box_2.clone());
    // }

    loop {
        let edge = edges
            .iter()
            .find(|e| !bin.is_connected(e.box_1.clone(), e.box_2.clone()));
        if edge.is_none() {
            break;
        }

        let edge = edge.unwrap();
        bin.merge(edge.box_1.clone(), edge.box_2.clone());
        last_edge = Some(edge);
    }

    if last_edge.is_none() {
        println!("Not find last edge.");
        return;
    }

    let last_edge = last_edge.unwrap();

    let result = last_edge.box_1.x * last_edge.box_2.x;
    println!("Part 2: {}", result);
}

fn main() {
    let contents = read_to_string("assets/2025/day_8.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let contents = contents.lines().collect::<Vec<&str>>();

    let boxes = contents
        .iter()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            Location {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect::<Vec<Location>>();

    // boxes.iter().for_each(|b| {
    //     println!("Box at x: {}, y: {}, z: {}", b.x, b.y, b.z);
    // });

    part_1(&boxes, 1000);
    part_2(&boxes);
}
