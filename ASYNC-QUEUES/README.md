# Async Queues Runtime

A minimal async task runtime with **priority-based scheduling** using custom `High` and `Low` queues. Built using `flume`, `async-task`, and `futures-lite`. I was trying to recreate some basic tokio async queues and most popular macros.

## Features

- Lightweight thread-based runtime
- High and Low priority task queues
- Thread-pool per priority queue
- Custom macro-based task spawning and joining (`spawn_task!`, `join!`, `try_join!`)
- No external executor dependencies like Tokio or async-std

