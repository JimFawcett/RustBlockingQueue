# RustBlockingQueue

https://JimFawcett.github.io/RustBlockingQueue.html

Thread safe queue that blocks dequeuer when empty

__Concept:__

  RustBlockingQueue is a facility for communicating between threads using a thread-safe blocking queue.  Note that
  the Rust message-passing facility does about the same thing.

  This is a nice illustration of how to build a data structure that can be shared between threads.  I intend to compare
  performance of this facility with message passing some time soon.

__Design:__

  <img src="https://JimFawcett.github.io/Pictures/BlockingQDiagram.JPG" width="500" />
  
  There is one struct, BlockingQueue&lt;T&gt;, with a few methods in this design:
  
    Methods:
      1. new() -> Self
           Create new empty BlockingQueue<T>.
 
      2. en_q(&self, t: T) -> Result<()>
          Push_back t onto internal VecDec<T>.
  
      3. de_q(&self) -> Result<T>;
           Pop_front t from internal VecDec<T>.

      4. len(&self) -> usize
           Return number of elements stored in queue.

<t-b>
    Sharing between threads is only possible, due to rules of the Rust language, if the shared items are 
    all Mutexes or Condvars, or an aggregate of those, e.g., a tuple, or struct like BlockingQueue.
  </t-b>
  <t-b>
    An instance of BlockingQueue&lt;T&gt; can be shared between threads because it only has two fields
    and those are share-able.  One is a Mutex&lt;VecDeque&lt;T&gt;&gt;, and the other is a Condvar,
    e.g., a condition variable.
  </t-b>
</t-b>
<div class="clear"></div>
<h3>Operation:</h3>
<t-b class="indent">
  Operation is illustrated by the test1.rs in /examples.
</t-b>
<h3>Build:</h3>
<t-b class="indent">
  Download and, in a command prompt, <c-s>cargo build</c-s> or <c-s>cargo run</c-s>.
</t-b>
