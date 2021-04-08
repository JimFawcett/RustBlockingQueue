# RustBlockingQueue

https://JimFawcett.github.io/RustBlockingQueue.html

Thread safe queue that blocks dequeuer when empty

##Concept:

  RustBlockingQueue is a facility for communicating between threads using a thread-safe blocking queue.  Note that
  the Rust message-passing facility does about the same thing.

  This is a nice illustration of how to build a data structure that can be shared between threads.  I intend to compare
  performance of this facility with message passing some time soon.

##Design:

  <img src="https://JimFawcett.github.io/Pictures/BlockingQDiagram.JPG" width="500" />
  
  There is one struct, BlockingQueue&lt;T&gt;, with a few methods in this design:
  
    Methods:
      1. ##new() -> Self
          Create new BlockingQueue which is empty.
 
      2. <strong><c-s>en_q(&self, t: T) -> Result<()></()></c-s></strong>
        <div style="padding:3px 10px 5px 10px;">
          Push_back t onto internal VecDec&lt;T&gt;.
        </div>
      3. <strong><c-s>de_q(&self) -> Result&lt;T&gt;</c-s></strong>
        <div style="padding:3px 10px 5px 10px;">
          Pop_front t from internal VecDec&lt;T&gt;.
        </div>
      4. <strong><c-s>len(&self) -> usize</c-s></strong>
        <div style="padding:3px 10px 5px 10px;">
          Return number of elements stored in queue.
        </div>

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
