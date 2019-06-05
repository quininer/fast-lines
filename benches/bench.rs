use std::io::{ BufRead, BufReader, Cursor };
use criterion::{ criterion_main, criterion_group, Criterion, black_box };
use fast_lines::ReadLine;

const DATA: &[u8] = include_bytes!("../src/lib.rs");


fn bench_fast(c: &mut Criterion) {
    c.bench_function("fast", |b| {
        b.iter(|| {
            let reader = Cursor::new(black_box(DATA));
            let mut reader = ReadLine::new(reader);
            while let Ok(Some(line)) = reader.read_line() {
                black_box(line);
            }
        });
    });
}

fn bench_io_lines(c: &mut Criterion) {
    c.bench_function("io_lines", |b| {
        b.iter(|| {
            let reader = Cursor::new(black_box(DATA));
            let reader = BufReader::new(reader);
            for line in reader.lines() {
                if let Ok(line) = line {
                    black_box(line);
                }
            }
        });
    });
}

fn bench_io_read_until(c: &mut Criterion) {
    c.bench_function("io_read_until", |b| {
        b.iter(|| {
            let reader = Cursor::new(black_box(DATA));
            let mut reader = BufReader::new(reader);
            let mut line = Vec::new();
            while let Ok(n) = reader.read_until(b'\n', &mut line) {
                if n > 0 {
                    black_box(&line);
                    line.clear();
                } else {
                    break;
                }
            }
        });
    });
}

criterion_group!(read_lines, bench_fast, bench_io_lines, bench_io_read_until);
criterion_main!(read_lines);
