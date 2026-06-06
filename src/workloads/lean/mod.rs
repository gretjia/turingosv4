use super::benchmark_boundary::WorkloadAdapterKind;

/// TRACE_MATRIX FC3: Lean adapter namespace identifies workload evidence without kernel authority.
pub fn adapter_kind() -> WorkloadAdapterKind {
    WorkloadAdapterKind::Lean
}
