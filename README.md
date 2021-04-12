# RustBlockingQueue

https://JimFawcett.github.io/RustBlockingQueue.html

Thread safe queue that blocks dequeuer when empty

__Concept:__

  RustBlockingQueue is a facility for communicating between threads using a thread-safe blocking queue.  Note that
  the Rust message-passing facility does about the same thing.

  This is a nice illustration of how to build a data structure that can be shared between threads.

__Design:__

  <img src="https://JimFawcett.github.io/Pictures/BlockingQDiagram.JPG" width="400" />
  
  There is one struct, BlockingQueue&lt;T&gt;, with a few methods in this design:
  
    Methods:
```rust
      1. new() -> Self
           Create new empty BlockingQueue<T>.
 
      2. en_q(&self, t: T) -> Result<()>
           Push_back t into internal VecDec<T>.
  
      3. de_q(&self) -> Result<T>;
           Pop_front t from internal VecDec<T>.

      4. len(&self) -> usize
           Return number of elements stored in queue.
```
    Sharing between threads is only possible, due to rules of the Rust language, if the shared items are 
    all Mutexes or Condvars, or an aggregate of those, e.g., a tuple, or struct like BlockingQueue.

    An instance of BlockingQueue<T> can be shared between threads because it only has two fields
    and those are share-able.  One is a Mutex<VecDeque<T>>, and the other is a Condvar,
    e.g., a condition variable.

<h3>Operation:</h3>

Operation is illustrated by the test1.rs in /examples.

<h3>Build:</h3>

Download and, in a command prompt, <c-s>cargo build</c-s> or <c-s>cargo test</c-s> or <c-s>cargo run --example test1.rs</c-s>.
