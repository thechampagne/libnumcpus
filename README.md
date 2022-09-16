# libnumcpus

[![](https://img.shields.io/github/v/tag/thechampagne/libnumcpus?label=version)](https://github.com/thechampagne/libnumcpus/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libnumcpus)](https://github.com/thechampagne/libnumcpus/blob/main/LICENSE)

Get the number of CPUs in **C**.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libnumcpus.git
```
#### 2. Navigate to the root
```
cd libnumcpus
```
#### 3. Build the project
```
cargo build
```

### API

```c
// Returns the number of available CPUs of the current system.
extern size_t num_cpus_get();

// Returns the number of physical cores of the current system.
extern size_t num_cpus_get_physical();
```

### References
 - [num_cpus](https://github.com/seanmonstar/num_cpus)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libnumcpus/blob/main/LICENSE).
