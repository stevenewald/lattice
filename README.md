# PostgreSQL Distributed Caching System

The PostgreSQL Distributed Caching System is a package designed to provide an easy-to-setup read-through and write-through distributed caching solution for PostgreSQL database queries. It aims to improve query performance by caching frequently accessed data in memory, reducing the need to hit the database for every query.

## Roadmap (in order, although will change)
1. Figure out key concepts
  - [x] Cache Invalidation
    - Make write-through required, although will need to find a way to enforce. Assuming good for now
  - [x] Memory Consumption
    - Handled by distributed nature. K8s automatically deploy addl nodes when required
  - [ ] Cache Coherency (/policy if not 100%)
    - Will have clients that use consistent hashing to read/write to distributed nodes.
    - Slight chance this affects invalidation. Idea: **centralized hash ring**
  - [ ] Eviction Policies
    - LRU?
    - [x] Eviction vs expansion
      - If LRU is hot, expand cluster (vs evict)
      - Synchronization may be complex
  - [ ] Query Complexity (which parts of query can be cached)
      - [ ] Do we want to modify queries in order to cache more data?
      - This is fundamental and **MUST** be decided early on
  - [x] Cache Warm-up and Cold-start
      - Can be decided later on
  - [x] Cache Consistency and Durability
      - No persistence + enforced read/write through so no biggie
2. Find package/data structure to store primitives/arrays
  - Do we want to store them with Rust DS or with another package
  - Maybe consider using redis? May be overkill but also may be v useful
3. Decide on preliminary file structure (annoying to do later on)
4. Implement loading configuration files (boring)
4. Implement basic sql querying from conf files (boring)
5+: k8s deployment, hashing/sharding, write-through, processing/caching, etc

## Features (eventually lol)

- [ ] In-memory caching: Data is stored in memory to enable fast retrieval and reduce latency.
- [ ] Read/Write-through: The cache synchronizes data with the underlying PostgreSQL database to ensure consistency.
- [ ] Distributed caching: The ability to distribute the cache across multiple nodes, enhancing performance and scalability.
- [ ] Simple setup: The package provides a user-friendly interface for quick and hassle-free integration into your application.
- [ ] Cache invalidation: Mechanisms to handle cache invalidation and ensure data integrity.

## Installation (also eventually)

To install the PostgreSQL Distributed Caching System, follow these steps:

1. Ensure you have PostgreSQL installed and running.
2. Clone the repository: `https://github.com/stevenewald/lattice.git`
3. Install the necessary dependencies and compile the binaries: `cargo build`
4. Configure the caching system with the appropriate settings, such as cache size, eviction policy, and distributed cache setup.
5. Start your application, and the caching system will automatically intercept and cache PostgreSQL queries.

## Configuration (eventually x3)

The caching system can be configured by modifying the provided configuration file (`config.js` or similar). Here are some of the key configuration options:

- Cache size: Specify the maximum number of entries or the amount of memory allocated for the cache.
- Eviction policy: Choose the cache eviction policy, such as LRU or LFU, to determine which data to remove when the cache reaches its capacity limit.
- Distributed cache: Configure the cache to be distributed across multiple nodes for improved performance and scalability.
- Cache invalidation strategy: Define how the cache should be invalidated when the underlying data changes in the PostgreSQL database.

## Usage

Once the PostgreSQL Distributed Caching System is installed and configured, it will automatically cache PostgreSQL queries. There's no need to modify your existing query code. The caching system intercepts queries and checks if the data is already available in the cache. If it is, the data is served directly from the cache, avoiding the need to hit the database.

## Contributing

Contributions to the PostgreSQL Distributed Caching System are welcome! If you encounter any issues, have feature requests, or would like to contribute enhancements, please open an issue or submit a pull request. Be sure to follow the project's code of conduct.

## License

The PostgreSQL Distributed Caching System is released under the [MIT License](LICENSE). Feel free to use, modify, and distribute this package in accordance with the terms of the license.

