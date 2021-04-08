# RustBlockingQueue

https://JimFawcett.github.io/RustBlockingQueue.html

Thread safe queue that blocks dequeuer when empty

    #Concept:

      RustBlockingQueue is a facility for communicating between threads using a thread-safe blocking queue.  Note that
      the Rust message-passing facility does about the same thing.
      <spacer-15></spacer-15>
      This is a nice illustration of how to build a data structure that can be shared between threads.  I intend to compare
      performance of this facility with message passing some time soon.

    #Design:
      <div style="width:calc(100vw - 6rem);">
        <photosizer-block src="Pictures/BlockingQDiagram.JPG" width="500" class="photoSizerBlock right" style="margin-top:0;">
          <span style="
          display: inline-block;
          font-weight: bold;
          font-family: 'Comic Sans MS, Tahoma';
          background-color: #ddd;
          width: 100%;
          padding: 5px 0px;
        ">
            Fig 1. BlockingQueue
          </span>
        </photosizer-block>
      </div>
    <t-b>
      <t-b>
        There is one struct, BlockingQueue&lt;T&gt;, with a few methods in this design:
        <indent-block class="pad5">
          Methods:
          <ol class="tight">
            <li>
              <strong><c-s>new() -> Self</c-s></strong>
              <div style="padding:3px 10px 5px 10px;">
                Create new <c-s>BlockingQueue</c-s> which is empty.
              </div>
            </li>
            <li>
              <strong><c-s>en_q(&self, t: T) -> Result<()></()></c-s></strong>
              <div style="padding:3px 10px 5px 10px;">
                Push_back t onto internal VecDec&lt;T&gt;.
              </div>
            </li>
            <li>
              <strong><c-s>de_q(&self) -> Result&lt;T&gt;</c-s></strong>
              <div style="padding:3px 10px 5px 10px;">
                Pop_front t from internal VecDec&lt;T&gt;.
              </div>
            </li>
            <li>
              <strong><c-s>len(&self) -> usize</c-s></strong>
              <div style="padding:3px 10px 5px 10px;">
                Return number of elements stored in queue.
              </div>
            </li>
          </ol>
        </indent-block>
      </t-b>
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
