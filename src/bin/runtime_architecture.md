# Mini Rust Runtime Architecture

This document visualizes the internal logic and workflow of the custom async runtime implemented in `mini_runtime.rs`, with a focus on distinguishing between the two types of "Poll".

## 1. Comparison: Rust Poll vs. OS Poll

It's critical to distinguish between these two different "Poll" mechanisms:

| Feature | Rust `Future::poll` | OS `mio::Poll::poll` |
| :--- | :--- | :--- |
| **Location** | `Executor` calling the `Task` | `Reactor` calling the OS Kernel |
| **Purpose** | Check if a task can make progress | Wait for *any* external I/O events |
| **Blocking** | **Non-blocking**. Must return quickly. | **Blocking**. Sleeps until events occur. |
| **Output** | `Ready(T)` or `Pending` | A list of ready `Events` |
| **Trigger** | Triggered by a `Waker` notification | Triggered by hardware/network activity |

---

## 2. Refined Sequence Diagram

This diagram separates the **Runtime-level Polling** from the **OS-level Polling**.

```mermaid
sequenceDiagram
    autonumber
    participant E as Executor (User Thread)
    participant F as Future (Task Logic)
    participant R as Reactor (MIO Thread)
    participant OS as OS Kernel (epoll/kqueue)

    Note over E,F: Phase 1: Task Execution (Non-blocking)
    E->>F: Rust-Poll: future.poll(cx)
    F->>F: Attempt I/O (e.g., read)
    F-->>E: return Poll::Pending (WouldBlock)
    F->>R: Register interest + Waker

    Note over R,OS: Phase 2: OS Waiting (Blocking)
    R->>OS: OS-Poll: mio.poll(events, timeout=None)
    Note right of OS: Thread sleeps here...
    OS-->>R: Network Event Ready!
    
    Note over R,E: Phase 3: Wakeup & Re-poll
    R->>E: waker.wake() (Task -> Ready Queue)
    E->>F: Rust-Poll: future.poll(cx) (AGAIN)
    F->>F: Attempt I/O (Success)
    F-->>E: return Poll::Ready(data)
```

## 3. Component Interaction Map

```mermaid
graph TD
    %% Define styles
    classDef rust fill:#f96,stroke:#333,stroke-width:2px;
    classDef os fill:#69f,stroke:#333,stroke-width:2px;

    subgraph "Executor Loop (Rust Layer)"
        Ex[Executor]
        Task[Future::poll]
    end

    subgraph "Reactor Loop (OS Layer)"
        Re[Reactor]
        Wait[mio::Poll::poll]
    end

    Ex -- "1. Call poll" --> Task
    Task -- "2. If Pending: Register" --> Re
    Re -- "3. Blocking Wait" --> Wait
    Wait -- "4. Interrupt/Event" --> Re
    Re -- "5. wake()" --> Ex

    class Task rust;
    class Wait os;
```

## 3. Component Details

- **Executor**: The engine that drives the execution. It continuously pops ready tasks from a queue and calls `poll` on them.
- **Reactor**: A background thread running `mio::Poll`. It monitors OS events and uses `Waker` to tell the Executor which tasks are ready to progress.
- **Task**: A wrapper around a pinned `Future` and a way to re-schedule itself via the `ready_queue`.
- **Waker**: The bridge between the Reactor and Executor. When the Reactor sees an event, it calls `wake()`, which puts the Task back into the Executor's queue.
