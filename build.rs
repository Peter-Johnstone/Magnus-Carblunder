use std::{
    fs::{self},
    io::Write,
    path::{Path},
};
use crate::table_gen::between_exclusive::BETWEEN_EXCLUSIVE;
use crate::table_gen::between_inclusive::BETWEEN_INCLUSIVE;
use crate::table_gen::line_bb::LINE_BB;

mod table_gen; // ./table_gen

fn write_if_changed(path: &Path, new_content: &str) {
    if let Ok(existing) = fs::read_to_string(path) {
        if existing == new_content {
            return; // Skip rewriting the file
        }
    }
    fs::write(path, new_content).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dir = Path::new("table_gen");
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    let out_dir = Path::new("src/attacks/tables");
    fs::create_dir_all(out_dir).unwrap();

    write_rook_attack_table(out_dir);
    write_rook_masks(out_dir);
    write_king_moves(out_dir);
    write_pawn_attacks(out_dir);
    write_knight_moves(out_dir);
    write_bishop_attack_table(out_dir);
    write_bishop_masks(out_dir);
    write_between_exclusive_table(out_dir);
    write_between_inclusive_table(out_dir);
    write_line_bb_table(out_dir);
    write_rays(out_dir);
}

fn write_bishop_attack_table(out_dir: &Path) {
    let table = table_gen::bishop::build_bishop_table();
    let dest = out_dir.join("bishop_attacks.rs");

    let mut content = String::new();
    content += "pub static BISHOP_ATTACKS: [[u64; 512]; 64] = [\n";
    for row in &table {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 511 {
                content += ", ";
            }
        }
        content += "],\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}




fn write_bishop_masks(out_dir: &Path) {
    write_u64_slice(
        out_dir,
        "bishop_masks.rs",
        "BISHOP_MASKS",
        &table_gen::bishop::BISHOP_MASKS,
    );
}

fn write_rays(out_dir: &Path) {
    let dest = out_dir.join("rays.rs");
    let mut content = String::new();

    let mut combined = [[0u64; 64]; 8];

    combined[..4].copy_from_slice(&table_gen::rook::ROOK_RAYS);
    combined[4..].copy_from_slice(&table_gen::bishop::BISHOP_RAYS);

    content += "pub static RAYS: [[u64; 64]; 8] = [\n";
    for (dir_idx, row) in combined.iter().enumerate() {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 63 {
                content += ", ";
            }
        }
        content += "]";
        if dir_idx != 7 {
            content += ",";
        }
        content += "\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}


fn write_rook_attack_table(out_dir: &Path) {
    let table = table_gen::rook::build_rook_table();
    let dest = out_dir.join("rook_attacks.rs");

    let mut content = String::new();
    content += "pub static ROOK_ATTACKS: [[u64; 4096]; 64] = [\n";
    for row in &table {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 4095 {
                content += ", ";
            }
        }
        content += "],\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}

fn write_between_exclusive_table(out_dir: &Path) {
    let dest = out_dir.join("between_exclusive.rs");


    let mut content = String::new();
    content += "pub static BETWEEN_EXCLUSIVE: [[u64; 64]; 64] = [\n";
    for row in &BETWEEN_EXCLUSIVE {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 4095 {
                content += ", ";
            }
        }
        content += "],\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}

fn write_line_bb_table(out_dir: &Path) {
    let dest = out_dir.join("line_bb.rs");


    let mut content = String::new();
    content += "pub static LINE_BB: [[u64; 64]; 64] = [\n";
    for row in &LINE_BB {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 4095 {
                content += ", ";
            }
        }
        content += "],\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}

fn write_between_inclusive_table(out_dir: &Path) {
    let dest = out_dir.join("between_inclusive.rs");

    let mut content = String::new();
    content += "pub static BETWEEN_INCLUSIVE: [[u64; 64]; 64] = [\n";
    for row in &BETWEEN_INCLUSIVE {
        content += "    [";
        for (i, v) in row.iter().enumerate() {
            content += &format!("0x{v:016X}u64");
            if i != 4095 {
                content += ", ";
            }
        }
        content += "],\n";
    }
    content += "];\n";

    write_if_changed(&dest, &content);
}

fn write_rook_masks(out_dir: &Path) {
    write_u64_slice(
        out_dir,
        "rook_masks.rs",
        "ROOK_MASKS",
        &table_gen::rook::ROOK_MASKS,
    );
}

fn write_king_moves(out_dir: &Path) {
    write_u64_slice(
        out_dir,
        "king_moves.rs",
        "KING_MOVES",
        &table_gen::king::king_table(),
    );
}

fn write_pawn_attacks(out_dir: &Path) {
    let dest   = out_dir.join("pawn_attacks.rs");
    let mut src = String::new();

    src.push_str("pub static PAWN_ATTACKS: [[u64; 64]; 2] = [\n");
    for (color_idx, row) in table_gen::pawn::pawn_attacks().iter().enumerate() {
        src.push_str("    [");
        for (i, v) in row.iter().enumerate() {
            src.push_str(&format!("0x{v:016X}u64"));
            if i != 63 { src.push_str(", "); }
        }
        src.push_str("]");
        if color_idx == 0 { src.push(','); }
        src.push('\n');
    }
    src.push_str("];\n");
    write_if_changed(&dest, &src);
}



fn write_knight_moves(out_dir: &Path) {
    write_u64_slice(
        out_dir,
        "knight_moves.rs",
        "KNIGHT_MOVES",
        &table_gen::knight::knight_table(),
    );
}

fn write_u64_slice(out_dir: &Path, file: &str, name: &str, slice: &[u64; 64]) {
    let mut content = String::new();
    content += &format!("pub static {name}: [u64; 64] = [\n");
    for (i, v) in slice.iter().enumerate() {
        if i % 8 == 0 {
            content += "    ";
        }
        content += &format!("0x{v:016X}u64");
        if i != 63 {
            content += ", ";
        }
        if i % 8 == 7 {
            content += "\n";
        }
    }
    content += "];\n";

    let path = out_dir.join(file);
    write_if_changed(&path, &content);
}
