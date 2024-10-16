use std::collections::HashMap;

use criterion::{Criterion, criterion_group, criterion_main};
use ulid::Ulid;

use storage::{create_workflow, get_non_queued_workflow, new_kv, Repository};
use storage::records::{Workflow, WorkflowStatus};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut kv = new_kv().expect("Can't create KV");

    c.bench_function("benchmark running heed storage write and get", |b| b.iter(|| {
        let map = HashMap::new();
        let map1 = HashMap::new();
        let ulid = Ulid::new();
        create_workflow(&mut kv, Workflow {
            name: "".to_string(),
            id: ulid,
            status: WorkflowStatus::Completed,
            queue_id: None,
            state: map,
            error: map1,
        }).expect("Cannot Write");
        let result = get_non_queued_workflow(&kv, &ulid.to_string()).expect("Cannot Read");
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
