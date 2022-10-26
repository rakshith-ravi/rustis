An asynchronous Redis client for Rust.

## Documentation
[Official Documentation](https://docs.rs/redis-driver/latest/redis_driver/)

## Philosophy
* Low allocations
* Full async library
* Rust idiomatic API

## Features
* Support all [Redis Commands](https://redis.io/commands/) (except Cluster related commands)
* Async support ([tokio](https://tokio.rs/) or [async-std](https://async.rs/))
* Different client types:
  * Single client
  * [Multiplexed](https://redis.com/blog/multiplexing-explained/) client
  * Pooled client manager (with [bb8](https://docs.rs/bb8/latest/bb8/))
* Automatic command batching
* Configuration with Redis URL or dedicated builder
* [TLS](https://redis.io/docs/manual/security/encryption/) support
* [Transaction](https://redis.io/docs/manual/transactions/) support
* [Pub/sub](https://redis.io/docs/manual/pubsub/) support
* [Sentinel](https://redis.io/docs/manual/sentinel/) support
* [LUA Scripts/Functions](https://redis.io/docs/manual/programmability/) support

## Roadmap
* [Cluster](https://redis.io/docs/manual/scaling/) support
* [Pipelining](https://redis.io/docs/manual/pipelining/) support
* [RedisJSON](https://redis.io/docs/stack/json/) support
