/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use num_cpus::get;
use num_cpus::get_physical;

/// Returns the number of available CPUs of the current system.
///
/// This function will get the number of logical cores. Sometimes this is different from the number
/// of physical cores See `Simultaneous multithreading on Wikipedia`.
///
/// This will always return at least `1`.
///
/// Example:
/// * *
/// int main()
/// {
///   size_t cpus = num_cpus_get();
///   if (cpus > 1)
///   {
///     printf("We are on a multicore system with #lu CPUs\n", cpus);
///   }
///   else
///   {
///     printf("We are on a single core system\n");
///   }
///   return 0;
/// }
/// * *
///
/// # Note
///
/// This will check `sched affinity` on Linux, showing a lower number of CPUs if the current
/// thread does not have access to all the computer's CPUs.
///
/// This will also check `cgroups`, frequently used in containers to constrain CPU usage.
#[no_mangle]
extern "C" fn num_cpus_get() -> usize {
  get()
}

/// Returns the number of physical cores of the current system.
///
/// This will always return at least `1`.
///
/// # Note
///
/// Physical count is supported only on Linux, mac OS and Windows platforms.
/// On other platforms, or if the physical count fails on supported platforms,
/// this function returns the same as `num_cpus_get()`, which is the number of logical
/// CPUS.
///
/// Example:
/// * *
/// int main()
/// {
///   size_t logical_cpus = num_cpus_get();
///   size_t physical_cpus = num_cpus_get_physical();
///   if (logical_cpus > physical_cpus)
///   {
///     printf("We have simultaneous multithreading with about %f \ logical cores to 1 physical core.\n", (double)logical_cpus / (double)physical_cpus);
///   }
///   else if (logical_cpus == physical_cpus)
///   {
///     printf("Either we don't have simultaneous multithreading, or our \ system doesn't support getting the number of physical CPUs.\n");
///   }
///   else
///   {
///     printf("We have less logical CPUs than physical CPUs, maybe we only have access to \ some of the CPUs on our system.\n");
///   }
///   return 0;
/// }
/// * *
#[no_mangle]
extern "C" fn num_cpus_get_physical() -> usize {
  get_physical()
}

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn num_cpus_get_test() {
    let cpus = num_cpus_get();
    if cpus > 1 {
      println!("We are on a multicore system with {} CPUs", cpus);
    } else {
      println!("We are on a single core system");
    }
  }

  #[test]
  fn num_cpus_get_physical_test() {
    let logical_cpus = num_cpus_get();
    let physical_cpus = num_cpus_get_physical();
    if logical_cpus > physical_cpus {
      println!("We have simultaneous multithreading with about {:.2} \
              logical cores to 1 physical core.", 
              (logical_cpus as f64) / (physical_cpus as f64));
    } else if logical_cpus == physical_cpus {
      println!("Either we don't have simultaneous multithreading, or our \
              system doesn't support getting the number of physical CPUs.");
    } else {
      println!("We have less logical CPUs than physical CPUs, maybe we only have access to \
              some of the CPUs on our system.");
    }
  }
}