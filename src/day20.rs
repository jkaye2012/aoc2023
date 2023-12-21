use std::collections::{HashMap, VecDeque};

const TYPE_MASK: u32 = 0xF0000000;
const FF_MASK: u32 = 0x1;
const CONJ_STATE_MASK: u32 = 0xFFF;
const CONJ_STABLE_MASK: u32 = 0xFFFF0000;

fn flip_flop_recv(ff: u32, pulse: u32) -> (u32, u32) {
    let s = !(ff | !FF_MASK) ^ (pulse & FF_MASK);
    let p = (s.wrapping_sub(1) ^ 0xFFF) & 0xFFFF0FFF;

    ((ff & CONJ_STABLE_MASK) | s, p | CONJ_STABLE_MASK)
}

fn conjunction_recv(conj: u32, pulse: u32) -> (u32, u32) {
    let all = (conj >> 16) & CONJ_STATE_MASK;
    let state = conj & all;
    let pulse_loc = (pulse >> 16) & CONJ_STATE_MASK;
    let pulse_val = pulse & pulse_loc;
    let new_state = (state & !pulse_loc) | pulse_val;
    //println!("{}, {}", all, new_state);
    let new_pulse = (all != new_state) as u32;
    let new_pulse = (new_pulse.wrapping_sub(1) ^ 0xFFF) & 0xFFFF0FFF;
    (
        (conj & CONJ_STABLE_MASK) | new_state,
        new_pulse | CONJ_STABLE_MASK,
    )
}

fn transition_index(state: u32) -> usize {
    ((state >> 28) - 1) as usize
}

pub struct Relay {
    states: Vec<(u32, Vec<(usize, u32)>)>,
    pulses: VecDeque<(usize, u32)>,
}

#[aoc_generator(day20)]
pub fn generate(input: &str) -> Relay {
    // let sstates = vec![
    //     (0, vec![]),
    //     (0x10000000, vec![(2, 0x00000001)]),
    //     (0x10000000, vec![(3, 0x00000001)]),
    //     (0x10000000, vec![(4, 0x00010001)]),
    //     (0x20010000, vec![(1, 0x00000001)]),
    // ];
    //let spulses = vec![(1, 0x00000000), (2, 0x00000000), (3, 0x00000000)].into();
    //    println!("{:?}", sstates);

    // let states = vec![
    //     (0, vec![]),
    //     (0x10000000, vec![(2, 0x00010001), (4, 0x00010001)]),
    //     (0x20010000, vec![(3, 0x00000001)]),
    //     (0x10000000, vec![(4, 0x00020003)]),
    //     (0x20030000, vec![(5, 0x00000001)]),
    //     (0, vec![]),
    // ];
    // let pulses = vec![(1, 0x00000000)].into();
    let mut ids = HashMap::new();
    let mut states = vec![(0, vec![])];
    input
        .lines()
        .skip(1)
        .map(|l| {
            let (id, _) = l.split_once(" -> ").unwrap();
            id
        })
        .enumerate()
        .for_each(|(idx, id)| {
            let is_conj = id.starts_with('&');
            ids.insert(&id[1..], (idx + 1, is_conj));
            let t = if is_conj { 0x20000000 } else { 0x10000000 };
            states.push((t, vec![]));
        });

    let mut lines = input.lines();
    let pulses = lines
        .next()
        .unwrap()
        .split_once(" -> ")
        .unwrap()
        .1
        .split(", ")
        .map(|id| (ids[id].0, 0x00000000))
        .collect();
    lines.enumerate().for_each(|(in_idx, l)| {
        let (_, out_ids) = l.split_once(" -> ").unwrap();

        for id in out_ids.split(", ") {
            let slen = states.len();
            let (idx, is_conj) = ids.get(id).copied().unwrap_or((slen, false));
            if is_conj {
                let mask = {
                    let mst = &mut states[idx];
                    let mut n = (mst.0 >> 16) & CONJ_STATE_MASK;
                    let mut mask = 0;
                    n <<= 1;
                    n |= 1;
                    mask |= (n + 1) << 15;
                    mask |= n;
                    mst.0 |= n << 16;
                    mask
                };
                states[in_idx + 1].1.push((idx, mask));
            } else {
                states[in_idx + 1].1.push((idx, 0x00000001));
            }
        }
    });
    states.push((0x40000000, vec![]));

    Relay { states, pulses }
}

fn press_button(
    states: &mut Vec<(u32, Vec<(usize, u32)>)>,
    pulses: &mut VecDeque<(usize, u32)>,
) -> (usize, usize, bool) {
    let mut high = 0;
    let mut low = 1;
    while let Some((dest, pulse)) = pulses.pop_front() {
        let state = states[dest].0;
        //  println!("processing: -{:#08x}-> {} {:#08x}", pulse, dest, state);

        let high_low = pulse & CONJ_STATE_MASK;
        high += (high_low != 0) as usize;
        low += (high_low == 0) as usize;

        let t = (state & TYPE_MASK) >> 28;
        if t == 4 && high_low == 0 {
            return (low, high, true);
        } else if !(t == 2 || (t == 1 && high_low == 0)) {
            continue;
        }

        let (new_state, new_pulse) = {
            [flip_flop_recv(state, pulse), conjunction_recv(state, pulse)][transition_index(state)]
        };
        // if transition_index(state) == 1 {
        //     println!(
        //         "{:#08x}, {:#08x}, {:#08x}, {:#08x}",
        //         state, pulse, new_state, new_pulse
        //     );
        // }
        states[dest].0 = new_state;

        for (o, p) in &states[dest].1 {
            //       println!("{:#08x}, {:#08x}, {:#08x}", *p, new_pulse, new_state);
            pulses.push_back((*o, *p & new_pulse));
        }
    }

    (low, high, false)
}

#[aoc(day20, part1)]
pub fn button_1000(input: &Relay) -> usize {
    let mut pulses = VecDeque::new();
    let mut states = input.states.clone();
    //println!("{:?}", states);
    let mut total_high = 0;
    let mut total_low = 0;
    for _ in 0..1000 {
        pulses.extend(&input.pulses);
        let (low, high, _) = press_button(&mut states, &mut pulses);
        total_high += high;
        total_low += low;
    }

    total_high * total_low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_flop_logic() {
        let ff_off = 0u32;
        let ff_on = 1u32;
        assert_eq!(flip_flop_recv(0x10000000, 0), (0x10000001, 0xFFFF0FFF));
        assert_eq!(flip_flop_recv(ff_off, 0x1), (ff_off, 0xFFFF0000));
        assert_eq!(flip_flop_recv(ff_off, 0x0), (ff_on, 0xFFFF0FFF));
        assert_eq!(flip_flop_recv(ff_on, 0x1), (ff_on, 0xFFFF0FFF));
        assert_eq!(flip_flop_recv(ff_on, 0x0), (ff_off, 0xFFFF0000));
    }

    #[test]
    fn conjunction_logic() {
        assert_eq!(
            conjunction_recv(0x20030000, 0x00010001),
            (0x20030001, 0xFFFF0FFF)
        );
        assert_eq!(
            conjunction_recv(0x20030000, 0x00020003),
            (0x20030002, 0xFFFF0FFF)
        );
        assert_eq!(
            conjunction_recv(0x20030001, 0x00020003),
            (0x20030003, 0xFFFF0000)
        );
        assert_eq!(
            conjunction_recv(0x20070001, 0x00020003),
            (0x20070003, 0xFFFF0FFF)
        );
        assert_eq!(
            conjunction_recv(0x20070001, 0x00040007),
            (0x20070005, 0xFFFF0FFF)
        );
        assert_eq!(
            conjunction_recv(0x20070001, 0x00040007),
            (0x20070005, 0xFFFF0FFF)
        );
    }
}
