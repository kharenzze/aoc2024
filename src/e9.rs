use crate::numbers::n_natural_sum_range;
use crate::range::Range;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Input = String;
type Output1 = i64;
type Output2 = i64;

const DAY: usize = 9;

fn read_data(is_test: bool) -> Input {
  let extension = if is_test { "test.txt" } else { "txt" };
  let filename = format!("./resources/{}.{}", DAY, extension);
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let mut reader = BufReader::new(file);
  let mut buf = String::new();
  reader.read_to_string(&mut buf).unwrap();
  buf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum BlockType {
  #[default]
  Empty,
  Id(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Block {
  block_type: BlockType,
  range: Range,
}

fn parse_blocks(line: &str) -> Vec<Block> {
  let mut id_count = 0;
  let mut pos_count = 0;
  line
    .chars()
    .enumerate()
    .filter_map(|(i, c)| {
      let n = c.to_digit(10).unwrap() as i64;
      let mut block = Block::default();
      if i % 2 == 0 {
        block.block_type = BlockType::Id(id_count);
        id_count += 1;
      }
      block.range = Range::new(pos_count, n as u64);
      pos_count += n;
      if n > 0 {
        Some(block)
      } else {
        None
      }
    })
    .collect()
}

fn sort_blocks(blocks: &Vec<Block>) -> Vec<Block> {
  let l = blocks.len();
  let mut sorted: Vec<Block> = Vec::with_capacity(l);
  let mut left = 0;
  let mut right = l - 1;
  let mut left_idx: i64 = 0;
  let mut current_left: Option<Block> = None;
  let mut current_right: Option<Block> = None;
  loop {
    if left >= right {
      if let Some(sender) = current_right {
        let to_be_consumed = sender.range.len;
        let block_to_inject = Block {
          block_type: sender.block_type,
          range: Range::new(left_idx, to_be_consumed),
        };
        sorted.push(block_to_inject);
      }
      break;
    }

    //find target block in left side
    if current_left.is_none() {
      let mut next = blocks[left];
      if let BlockType::Id(_) = next.block_type {
        next.range.start = left_idx;
        left_idx += next.range.len as i64;
        sorted.push(next);
        left += 1;
        continue;
      }
      current_left = Some(next);
    }

    //find target block in right side
    if current_right.is_none() {
      let next = blocks[right];
      if next.block_type == BlockType::Empty {
        right -= 1;
        continue;
      }
      current_right = Some(next);
    }

    let mut receiver = current_left.unwrap();
    let mut sender = current_right.unwrap();
    let to_be_consumed = receiver.range.len.min(sender.range.len);
    let block_to_inject = Block {
      block_type: sender.block_type,
      range: Range::new(left_idx, to_be_consumed),
    };
    sorted.push(block_to_inject);
    left_idx += to_be_consumed as i64;

    receiver.range.len -= to_be_consumed;
    if receiver.range.len == 0 {
      left += 1;
      current_left = None;
    } else {
      current_left = Some(receiver);
    }

    sender.range.len -= to_be_consumed;
    if sender.range.len == 0 {
      right -= 1;
      current_right = None;
    } else {
      current_right = Some(sender);
    }
  }

  sorted
}

fn sort_blocks_v2(blocks: &Vec<Block>) -> Vec<Block> {
  let mut bmap: BTreeMap<i64, Block> = BTreeMap::new();

  blocks.iter().for_each(|&block| {
    bmap.insert(block.range.start, block);
  });

  for &block in blocks.iter().rev() {
    let BlockType::Id(id) = block.block_type else {
      continue;
    };

    let current_block_len = block.range.len;

    //find a place to insert it in the map
    let gap = bmap
      .values()
      .take_while(|b| {
        if let BlockType::Id(gap_id) = b.block_type {
          return gap_id != id;
        }
        true
      })
      .filter(|b| b.block_type == BlockType::Empty)
      .find(|b| {
        let b_len = b.range.len;
        b_len >= current_block_len
      });

    let Some(gap) = gap else {
      continue;
    };

    let gap_pos = gap.range.start;
    let mut current_block = bmap.remove(&block.range.start).unwrap();

    //remove the gap and insert the current block
    let gap = bmap.remove(&gap_pos).unwrap();
    current_block.range.start = gap_pos;
    bmap.insert(gap_pos, current_block);
    let diff = gap.range.len - current_block_len;
    if diff > 0 {
      let gap_pos = gap.range.start + current_block_len as i64;
      bmap.insert(
        gap_pos,
        Block {
          block_type: BlockType::Empty,
          range: Range::new(gap_pos, diff as u64),
        },
      );
    }
  }

  let sorted: Vec<Block> = bmap.into_values().collect();

  sorted
}

fn compute_score(blocks: &Vec<Block>) -> i64 {
  let mut score = 0;
  for block in blocks {
    if let BlockType::Id(id) = block.block_type {
      score += id * n_natural_sum_range(block.range);
    }
  }
  score
}

fn initial(input: Input) -> Output1 {
  let blocks = parse_blocks(&input);
  let sorted = sort_blocks(&blocks);
  compute_score(&sorted)
}

fn extra(input: Input) -> Output2 {
  let blocks = parse_blocks(&input);
  let sorted = sort_blocks_v2(&blocks);
  compute_score(&sorted)
}

pub fn solve(part: usize) {
  let input = read_data(false);
  let score = if part == 1 {
    initial(input)
  } else {
    extra(input)
  };
  println!("{score}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let input = read_data(true);
    let score = initial(input);
    assert_eq!(score, 13)
  }

  #[test]
  fn two() {
    let input = read_data(true);
    let score = extra(input);
    assert_eq!(score, 2858)
  }
}
