use rustc_hash::FxHashMap;

pub type PartRatings = [u16; 4];

// 16 bits: destination, 2 bits: rating to extract, 1 bit: gt or lt, 13 bits: rating threshold
#[derive(Clone, Copy, Debug)]
pub struct Workflow(u32);

const DESTINATION_MASK: u32 = 0xFFFF0000;
const RATING_MASK: u32 = 0x0000C000;
const COMPARISON_MASK: u32 = 0x00002000;
const THRESHOLD_MASK: u32 = 0x000001FFF;

impl Workflow {
    pub fn new(destination: usize, rating: u8, comparison: u8, threshold: u16) -> Self {
        let dest: u32 = destination.try_into().unwrap();
        Self(dest << 16 | (rating as u32) << 14 | (comparison as u32) << 13 | threshold as u32)
    }

    #[inline(always)]
    pub fn destination(&self) -> usize {
        ((self.0 & DESTINATION_MASK) >> 16) as usize
    }

    #[inline(always)]
    pub fn rating_index(&self) -> usize {
        ((self.0 & RATING_MASK) >> 14) as usize
    }

    #[inline(always)]
    pub fn comparison_index(&self) -> usize {
        ((self.0 & COMPARISON_MASK) >> 13) as usize
    }

    #[inline(always)]
    pub fn rating_threshold(&self) -> u16 {
        (self.0 & THRESHOLD_MASK) as u16
    }
}

pub struct System {
    workflows: Vec<Workflow>,
    parts: Vec<PartRatings>,
    start: usize,
}

#[aoc_generator(day19)]
pub fn generate(input: &str) -> System {
    let (raw_workflows, raw_parts) = input.split_once("\n\n").unwrap();
    let mut workflow_ids = FxHashMap::default();
    workflow_ids.insert("R", 0);
    workflow_ids.insert("A", 1);
    let mut pos = 1;
    raw_workflows.split('\n').for_each(|wk| {
        let (id, rest) = wk.split_once('{').unwrap();
        pos += 1;
        workflow_ids.insert(id, pos);
        pos += rest.chars().filter(|c| *c == ',').count();
    });
    let start = workflow_ids["in"];

    let mut workflows = vec![Workflow(0); pos - 1];
    let mut idx = 0;
    raw_workflows.split('\n').for_each(|rw| {
        let (_, wk) = rw.split_once('{').unwrap();
        wk.trim_end_matches('}').split(',').for_each(|w| {
            let workflow = if w.len() < 4 {
                let dest = workflow_ids[w];
                Workflow::new(dest, 0, 1, 0)
            } else {
                let (cond, dest) = w.split_once(':').unwrap();
                let bytes = cond.as_bytes();
                let rating = (bytes[0] - b'a') / 7;
                let comparison = bytes[1] / 62;
                let mut threshold: u16 = 0;
                for b in &bytes[2..] {
                    threshold *= 10;
                    threshold += (*b - b'0') as u16;
                }
                Workflow::new(workflow_ids[dest], rating, comparison, threshold)
            };
            workflows[idx] = workflow;
            idx += 1;
        });
    });

    let parts = raw_parts
        .split('\n')
        .map(|p| {
            let mut ratings = [0u16; 4];
            p[1..p.len() - 1]
                .split(',')
                .enumerate()
                .for_each(|(idx, threshold)| {
                    ratings[idx] = threshold[2..].parse::<u16>().unwrap();
                });
            ratings.swap(0, 3);
            ratings.swap(0, 2);
            ratings
        })
        .collect();

    System {
        workflows,
        parts,
        start,
    }
}

#[aoc(day19, part1)]
pub fn accepted(system: &System) -> usize {
    let mut total = 0usize;
    let mut dst_buf = [0usize; 2];
    let mut cmp_buf = [false; 2];
    for part in &system.parts {
        let mut idx = system.start;
        while idx > 1 {
            let workflow = system.workflows[idx - 2];
            let rating = part[workflow.rating_index()];
            let threshold = workflow.rating_threshold();
            dst_buf[0] = idx + 1;
            dst_buf[1] = workflow.destination();
            cmp_buf[0] = rating < threshold;
            cmp_buf[1] = rating > threshold;
            idx = dst_buf[cmp_buf[workflow.comparison_index()] as usize];
        }
        for r in part {
            total += *r as usize * idx;
        }
    }
    total
}

type PartState = [(u16, u16); 4];

fn accepter(workflows: &[Workflow], idx: usize, state: PartState) -> usize {
    if idx == 0 {
        0
    } else if idx == 1 {
        let mut total = 1usize;
        for (lo, hi) in &state {
            total *= (hi - lo + 1) as usize;
        }
        total
    } else {
        let workflow = workflows[idx - 2];
        if workflow.rating_threshold() == 0 {
            accepter(workflows, workflow.destination(), state)
        } else {
            let (lo, hi) = state[workflow.rating_index()];
            let mut lt = state;
            lt[workflow.rating_index()] = (lo, workflow.rating_threshold());
            let mut gt = state;
            gt[workflow.rating_index()] = (workflow.rating_threshold(), hi);
            let (ltidx, gtidx) = if workflow.comparison_index() == 0 {
                lt[workflow.rating_index()].1 -= 1;
                (workflow.destination(), idx + 1)
            } else {
                gt[workflow.rating_index()].0 += 1;
                (idx + 1, workflow.destination())
            };

            accepter(workflows, gtidx, gt) + accepter(workflows, ltidx, lt)
        }
    }
}

#[aoc(day19, part2)]
pub fn all_accepted(system: &System) -> usize {
    accepter(&system.workflows, system.start, [(1, 4000); 4])
}
