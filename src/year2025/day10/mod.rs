use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct Day {
    machines: Vec<Machine>,
}

#[derive(Debug, Clone)]
struct Machine {
    buttons: Vec<Vec<usize>>, // list of indices toggled by each button
    light_indicators: Vec<u8>, // 0/1 vector desired
    joultages: Vec<u32>
}

// MatrixGF2: eine kleine, speziell auf GF(2) zugeschnittene Matrix-Hilfe.
// - Speichert die Einträge als Vec<Vec<u8>> (0/1)
// - Bietet Methoden zum Setzen/Lesen, Zeilen tauschen und Zeilen-xor ab einer Spalte.
// Ziel: den Code lesbarer und abstrakter machen, ohne das Verhalten zu verändern.
#[derive(Debug, Clone)]
struct MatrixGF2 {
    rows: usize,
    cols: usize,
    data: Vec<Vec<u8>>,
}

impl MatrixGF2 {
    /// Erzeuge eine rows x cols Matrix, initialisiert mit Nullen.
    fn new(rows: usize, cols: usize) -> Self {
        MatrixGF2 { rows, cols, data: vec![vec![0u8; cols]; rows] }
    }

    /// Setze Wert an (row, col) auf v (0 oder 1)
    fn set(&mut self, row: usize, col: usize, v: u8) {
        if row < self.rows && col < self.cols {
            self.data[row][col] = v & 1;
        }
    }

    /// Lese den Wert an (row, col)
    fn get(&self, row: usize, col: usize) -> u8 {
        if row < self.rows && col < self.cols {
            self.data[row][col]
        } else { 0 }
    }

    /// Tausche zwei Zeilen
    fn swap_rows(&mut self, a: usize, b: usize) {
        if a < self.rows && b < self.rows {
            self.data.swap(a, b);
        }
    }

    /// Führe: row_dst = row_dst XOR row_src, nur für Spalten >= start_col
    fn xor_rows_from(&mut self, row_dst: usize, row_src: usize, start_col: usize) {
        if row_dst >= self.rows || row_src >= self.rows { return; }
        let start = start_col.min(self.cols);
        for c in start..self.cols {
            self.data[row_dst][c] ^= self.data[row_src][c];
        }
    }
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut machines = Vec::new();

        for line in s.lines().map(|l| l.trim()) {
            if line.is_empty() {
                continue;
            }

            // parse lights inside [ ... ]
            let start = line.find('[').ok_or_else(|| crate::aoc::Error::Parse("missing [".to_string()))?;
            let end = line.find(']').ok_or_else(|| crate::aoc::Error::Parse("missing ]".to_string()))?;
            let lights_str = &line[start + 1..end];
            let light_indicators: Vec<u8> = lights_str
                .chars()
                .map(|c| match c {
                    '.' => 0u8,
                    '#' => 1u8,
                    _ => 0u8,
                })
                .collect();

            // parse button groups between end and the first { (joltage) or end of line
            let (rest, joultage_str) = if let Some(curly) = line.find('{') {
                (&line[end + 1..curly], &line[curly + 1..])
            } else {
                (&line[end + 1..] , &line[end + 1..])
            };

            let mut buttons: Vec<Vec<usize>> = Vec::new();
            let mut in_paren = false;
            let mut buf = String::new();
            for ch in rest.chars() {
                if ch == '(' {
                    in_paren = true;
                    buf.clear();
                } else if ch == ')' {
                    in_paren = false;
                    // parse numbers in buf (comma separated), skip empties
                    let mut btn: Vec<usize> = Vec::new();
                    for part in buf.split(',') {
                        let t = part.trim();
                        if t.is_empty() { continue; }
                        let v: usize = t.parse()?;
                        btn.push(v);
                    }
                    buttons.push(btn);
                } else if in_paren {
                    buf.push(ch);
                }
            }

            let mut joultages: Vec<u32> = Vec::new();
            if let Some(curly_end) = joultage_str.find('}') {
                let joultage_content = &joultage_str[..curly_end];
                for part in joultage_content.split(',') {
                    let t = part.trim();
                    if t.is_empty() { continue; }
                    let v: u32 = t.parse()?;
                    joultages.push(v);
                }
                // We currently do not use joultages, but they are parsed here.
            }

            machines.push(Machine { buttons, light_indicators, joultages });
        }

        Ok(Day { machines })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
            7,
        )]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut total: u64 = 0;

        for machine in &self.machines {
            let rows = machine.light_indicators.len();
            let columns = machine.buttons.len();

            // Erstelle eine Matrix (A) mit Zeilen = Anzahl Lichter, Spalten = Anzahl Tasten.
            // (A_{i,j}=1) genau dann, wenn Taste (j) Licht (i) toggelt.
            // Verwende die neue Matrix-Abstraktion
            let mut matrix = MatrixGF2::new(rows, columns);
            for (j, button) in machine.buttons.iter().enumerate() {
                for &button_index in button {
                    // Indices außerhalb der Zeilenanzahl ignorieren
                    if button_index < rows {
                        matrix.set(button_index, j, 1);
                    }
                }
            }

            // Gesucht ist ein Schaltvektor (x) (0/1 pro Taste) mit (A x = b) (mod 2).
            let mut solution = machine.light_indicators.clone();

            // Gaussian elimination over GF(2)
            // https://www.cs.umd.edu/~gasarch/TOPICS/factoring/fastgauss.pdf
            let mut row = 0usize;
            let mut pivot_for_column: Vec<Option<usize>> = vec![None; columns];

            for col in 0..columns {
                let mut selected = None;
                for current_row in row..rows {
                    if matrix.get(current_row, col) == 1 {
                        selected = Some(current_row);
                        break;
                    }
                }
                if let Some(selr) = selected {
                    matrix.swap_rows(row, selr);
                    solution.swap(row, selr);
                    for r in 0..rows {
                        if r != row && matrix.get(r, col) == 1 {
                            matrix.xor_rows_from(r, row, col);
                            solution[r] ^= solution[row];
                        }
                    }
                    pivot_for_column[col] = Some(row);
                    row += 1;
                    if row == rows {
                        break;
                    }
                }
            }

            let mut is_free: Vec<bool> = vec![false; columns];
            for c in 0..columns {
                if pivot_for_column[c].is_none() {
                    is_free[c] = true;
                }
            }
            let free_indices: Vec<usize> = is_free.iter().enumerate().filter(|(_, &f)| f).map(|(i, _)| i).collect();
            let d = free_indices.len();

            let mut pivot_rows: Vec<(usize, Vec<u8>, u8)> = Vec::new();
            for c in 0..columns {
                if let Some(pivot_column) = pivot_for_column[c] {
                    let mut coeffs = Vec::with_capacity(d);
                    for &free_index in &free_indices {
                        coeffs.push(matrix.get(pivot_column, free_index));
                    }
                    pivot_rows.push((c, coeffs, solution[pivot_column]));
                }
            }

            // Set all free vars = 0
            let mut particular = vec![0u8; columns];
            for (pivot_col, _coeffs, rhs) in pivot_rows.iter() {
                particular[*pivot_col] = *rhs;
            }

            // Nullspace basis vectors: for each free var k, set that free var=1 and others 0
            let mut null_basis: Vec<Vec<u8>> = Vec::new();
            for (k_idx, &free_col) in free_indices.iter().enumerate() {
                let mut vec_k = vec![0u8; columns];
                vec_k[free_col] = 1;
                for (pivot_col, coeffs, _rhs) in pivot_rows.iter() {
                    if coeffs[k_idx] == 1 {
                        vec_k[*pivot_col] = 1;
                    }
                }
                null_basis.push(vec_k);
            }

            // Enumerate all combinations to find minimal weight
            let mut best = None::<usize>;

            let combos = 1usize << d;
            for mask in 0..combos {
                let mut sol = particular.clone();
                for k in 0..d {
                    if (mask >> k) & 1 == 1 {
                        for i in 0..columns {
                            sol[i] ^= null_basis[k][i]; // add null_basis[k]
                        }
                    }
                }
                let weight = sol.iter().map(|&v| v as usize).sum();
                match best {
                    None => best = Some(weight),
                    Some(x) => if weight < x { best = Some(weight); }
                }
            }

            let min_presses = best.unwrap_or(0) as u64;
            total += min_presses;
        }

        Ok(total)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        Ok(0)
    }
}
