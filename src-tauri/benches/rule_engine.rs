use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use ai_security_scanner_lib::adapters::control_mapping::{lookup, source_rule_matches, MappingEntry, MatchKind};
use ai_security_scanner_lib::workspace_snapshot::{create_workspace_snapshot, WorkspaceSnapshotLimits};
use std::fs;

use tempfile::TempDir;

fn bench_rule_pattern_evaluation(c: &mut Criterion) {
    c.bench_function("lookup exact match", |b| {
        b.iter(|| {
            lookup("kics", "11111111-1111-1111-1111-111111111111", false, false)
        })
    });

    c.bench_function("lookup prefix match", |b| {
        b.iter(|| {
            lookup("garak", "prompt_inject.Probe/some_detector", false, false)
        })
    });

    c.bench_function("lookup miss", |b| {
        b.iter(|| {
            lookup("kics", "nonexistent-rule", false, false)
        })
    });

    let entry = MappingEntry {
        engine_id: "test".to_string(),
        match_kind: MatchKind::Prefix,
        source_rule: "prefix_".to_string(),
        controls: vec!["control1".to_string()],
        rationale: "rationale".to_string(),
    };
    c.bench_function("source_rule_matches prefix", |b| {
        b.iter(|| {
            source_rule_matches(&entry, "prefix_match_string")
        })
    });
}

fn create_mock_directory_structure(root: &std::path::Path, depth: usize, width: usize, files_per_dir: usize) {
    if depth == 0 {
        return;
    }
    for i in 0..width {
        let dir_path = root.join(format!("dir_{}", i));
        fs::create_dir_all(&dir_path).unwrap();
        for j in 0..files_per_dir {
            let file_path = dir_path.join(format!("file_{}.txt", j));
            fs::write(file_path, "test file content").unwrap();
        }
        create_mock_directory_structure(&dir_path, depth - 1, width, files_per_dir);
    }
}

fn bench_file_traversal(c: &mut Criterion) {
    let source_dir = TempDir::new().unwrap();
    create_mock_directory_structure(source_dir.path(), 3, 3, 5);

    c.bench_function("create_workspace_snapshot", |b| {
        b.iter_batched(
            || {
                let artifact_root = TempDir::new().unwrap();
                artifact_root
            },
            |artifact_root| {
                let _ = create_workspace_snapshot(
                    artifact_root.path(),
                    "test_case_id",
                    "test_source_id",
                    source_dir.path(),
                    WorkspaceSnapshotLimits::default(),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_rule_pattern_evaluation, bench_file_traversal);
criterion_main!(benches);
