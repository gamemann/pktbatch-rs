/// Retrieves the total numbers of CPUs available on the system.
///
/// NOTE: This uses the `num_cpus` crate to retrieve the CPU counts.
///
/// # Returns
/// The total number of CPUs available on the system.
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}
