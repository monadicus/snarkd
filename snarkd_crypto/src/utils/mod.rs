pub mod poseidon;
pub mod sha256;
pub use poseidon::*;

pub struct ExecutionPool<'a, T> {
    jobs: Vec<Box<dyn 'a + FnOnce() -> T + Send>>,
}

impl<'a, T> ExecutionPool<'a, T> {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            jobs: Vec::with_capacity(cap),
        }
    }

    pub fn add_job<F: 'a + FnOnce() -> T + Send>(&mut self, f: F) {
        self.jobs.push(Box::new(f));
    }

    pub fn execute_all(self) -> Vec<T>
    where
        T: Send + Sync,
    {
        use rayon::prelude::*;
        execute_with_max_available_threads(|| self.jobs.into_par_iter().map(|f| f()).collect())
    }
}

impl<'a, T> Default for ExecutionPool<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn max_available_threads() -> usize {
    let rayon_threads = rayon::current_num_threads();

    match get_cpu() {
        Cpu::Intel => num_cpus::get_physical().min(rayon_threads),
        Cpu::AMD | Cpu::Unknown => rayon_threads,
    }
}

pub fn execute_with_max_available_threads<T: Sync + Send>(f: impl FnOnce() -> T + Send) -> T {
    execute_with_threads(f, max_available_threads())
}

fn execute_with_threads<T: Sync + Send>(f: impl FnOnce() -> T + Send, num_threads: usize) -> T {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    pool.install(f)
}

/// Creates parallel iterator over refs if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_iter {
    ($e: expr) => {{
        let result = $e.par_iter();

        result
    }};
}

/// Creates parallel iterator over mut refs if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_iter_mut {
    ($e: expr) => {{
        let result = $e.par_iter_mut();

        result
    }};
}

/// Creates parallel iterator if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_into_iter {
    ($e: expr) => {{
        let result = $e.into_par_iter();

        result
    }};
}

/// Returns an iterator over `chunk_size` elements of the slice at a
/// time.
#[macro_export]
macro_rules! cfg_chunks {
    ($e: expr, $size: expr) => {{
        let result = $e.par_chunks($size);

        result
    }};
}

/// Returns an iterator over `chunk_size` elements of the slice at a time.
#[macro_export]
macro_rules! cfg_chunks_mut {
    ($e: expr, $size: expr) => {{
        let result = $e.par_chunks_mut($size);

        result
    }};
}

/// Applies the reduce operation over an iterator.
#[macro_export]
macro_rules! cfg_reduce {
    ($e: expr, $default: expr, $op: expr) => {{
        let result = $e.reduce($default, $op);

        result
    }};
}

/// Uses Rust's `cpuid` function from the `arch` module.
pub(crate) mod native_cpuid {
    /// Low-level data-structure to store result of cpuid instruction.
    #[derive(Copy, Clone, Eq, PartialEq)]
    #[repr(C)]
    pub struct CpuIdResult {
        /// Return value EAX register
        pub eax: u32,
        /// Return value EBX register
        pub ebx: u32,
        /// Return value ECX register
        pub ecx: u32,
        /// Return value EDX register
        pub edx: u32,
    }

    #[allow(unreachable_code)]
    pub fn cpuid_count() -> CpuIdResult {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            #[cfg(all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"))]
            use core::arch::x86 as arch;
            #[cfg(all(target_arch = "x86_64", not(target_env = "sgx")))]
            use core::arch::x86_64 as arch;

            // Safety: CPUID is supported on all x86_64 CPUs and all x86 CPUs with SSE, but not by SGX.
            let result = unsafe { arch::__cpuid_count(0, 0) };
            return CpuIdResult {
                eax: result.eax,
                ebx: result.ebx,
                ecx: result.ecx,
                edx: result.edx,
            };
        }

        CpuIdResult {
            eax: 22,
            ebx: 1970169159,
            ecx: 1818588270,
            edx: 1231384169,
        }
    }
}

///
/// Vendor Info String (LEAF=0x0)
///
/// The vendor info is a 12-byte (96 bit) long string stored in `ebx`, `edx` and
/// `ecx` by the corresponding `cpuid` instruction.
///
#[derive(PartialEq, Eq)]
#[repr(C)]
struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl VendorInfo {
    /// Return vendor identification as string, such as "AuthenticAMD" or "GenuineIntel".
    fn as_str(&self) -> &str {
        let brand_string_start = self as *const VendorInfo as *const u8;
        let slice = unsafe {
            // Safety: VendorInfo is laid out with repr(C) and exactly
            // 12 byte long without any padding.
            core::slice::from_raw_parts(brand_string_start, core::mem::size_of::<VendorInfo>())
        };
        core::str::from_utf8(slice).unwrap_or("InvalidVendorString")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cpu {
    AMD,
    Intel,
    Unknown,
}

///
/// Returns a new Cpu enum.
///
/// The vendor leaf will contain a ASCII readable string such as "GenuineIntel"
/// for Intel CPUs or "AuthenticAMD" for AMD CPUs.
///
#[allow(clippy::absurd_extreme_comparisons)]
pub fn get_cpu() -> Cpu {
    const EAX_VENDOR_INFO: u32 = 0x0;

    // Check if a non extended leaf  (`val`) is supported.
    let vendor_leaf = native_cpuid::cpuid_count();
    let is_leaf_supported = EAX_VENDOR_INFO <= vendor_leaf.eax;

    if is_leaf_supported {
        let vendor = VendorInfo {
            ebx: vendor_leaf.ebx,
            ecx: vendor_leaf.ecx,
            edx: vendor_leaf.edx,
        };

        match vendor.as_str() {
            "AuthenticAMD" => Cpu::AMD,
            "GenuineIntel" => Cpu::Intel,
            _ => Cpu::Unknown,
        }
    } else {
        Cpu::Unknown
    }
}
