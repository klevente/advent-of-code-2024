use advent_of_code_2024::read_file_to_string;

fn calculate_compact_filesystem_checksum_with_simple_algorithm(disk_map_str: &str) -> usize {
    let mut disk_map = disk_map_str
        .chars()
        .map(|c| c.to_digit(10).expect("Input should only have digits"))
        .collect::<Vec<_>>();

    let mut result = 0usize;
    let mut position_on_disk = 0;

    let mut front_idx = 0usize;
    let mut back_idx = disk_map.len() - 1;
    if back_idx % 2 == 1 {
        // If we end with an empty space, move back to last file
        back_idx -= 1;
    }

    loop {
        if front_idx % 2 == 0 {
            // Even: block (0, 2, 4, ..)
            let file_id = front_idx / 2;
            let block_size = disk_map[front_idx];
            if block_size == 0 {
                break;
            }
            for _ in 0..block_size {
                result += file_id * position_on_disk;
                position_on_disk += 1;
            }
            disk_map[front_idx] = 0;
        } else {
            // Odd: empty (1, 3, 5, ..)
            let empty_block_size = disk_map[front_idx];
            let mut file_id = back_idx / 2;

            for _ in 0..empty_block_size {
                if disk_map[back_idx] == 0 {
                    back_idx -= 2;
                    if back_idx <= front_idx {
                        break;
                    }
                    file_id = back_idx / 2;
                }
                result += file_id * position_on_disk;
                position_on_disk += 1;
                disk_map[back_idx] -= 1;
            }
        }
        front_idx += 1;
    }
    result
}

fn calculate_compact_filesystem_checksum_with_full_file_algorithm(disk_map_str: &str) -> usize {
    let disk_map = disk_map_str
        .chars()
        .map(|c| c.to_digit(10).expect("Input should only have digits") as usize)
        .collect::<Vec<_>>();

    let (file_blocks, mut empty_blocks) = {
        let mut file_blocks = Vec::new();
        let mut empty_blocks = Vec::new();
        let mut position = 0;
        for i in 0..disk_map.len() {
            if i % 2 == 1 {
                // Odd: empty block
                empty_blocks.push((disk_map[i], position))
            } else {
                // Even: file block
                file_blocks.push(position);
            }
            position += disk_map[i];
        }

        (file_blocks, empty_blocks)
    };

    let mut result = 0;

    for i in (0..disk_map.len()).rev().step_by(2) {
        let file_id = i / 2;
        let file_block_size = disk_map[i];

        if let Some(idx_of_empty_block_with_enough_space) = empty_blocks
            .iter()
            .take(file_id)
            .position(|(empty_block_size, _)| *empty_block_size >= file_block_size)
        {
            for k in 0..file_block_size {
                result += (empty_blocks[idx_of_empty_block_with_enough_space].1 + k) * file_id;
            }

            empty_blocks[idx_of_empty_block_with_enough_space].0 -= file_block_size;
            empty_blocks[idx_of_empty_block_with_enough_space].1 += file_block_size;
        } else {
            for k in 0..file_block_size {
                result += (file_blocks[file_id] + k) * file_id
            }
        }
    }

    result
}

fn main() {
    let disk_map = read_file_to_string("input/day9.txt");
    let compact_filesystem_checksum_simple =
        calculate_compact_filesystem_checksum_with_simple_algorithm(&disk_map);
    println!("The checksum of the compact filesystem using the simple algorithm is {compact_filesystem_checksum_simple}");

    let compact_filesystem_checksum_full_file =
        calculate_compact_filesystem_checksum_with_full_file_algorithm(&disk_map);
    println!("The checksum of the compact filesystem using the full file algorithm is {compact_filesystem_checksum_full_file}");
}
