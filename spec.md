Dequeasm (Pronounced "The Chasm" because it will suck you into a pit of dispair) is a deque based programming language with syntax similar to assembly. It is recommended to have some familiarity with [[Deque|deque]], [[Queue|dueue]] or [[Stack|stack]] data structures aswell as basic to intermediate understanding of assembly.

==Syntax Concepts==
Dequeasm is very similar to assembly in syntax and tries to mirror it as closely as possible. One major change however is that All commands in Dequeasm are followed by or proceeded by a <code>~</code> to indicate weather to operate on the front or back of the deque. An intuitive way to understand this is the following example:
<pre>
~PSH 1 ; pushes 1 onto the left of the stack
PSH~ 1 ; pushes 1 onto the right of the stack
</pre>
If such syntax is not used for commands that require a direction, Dequeasm will throw a syntax error and guide you to the line with the faulty command. As seen above, <code>;</code> can be used like in assembly to mark the start of a comment.<br>
Another similarity to assembly is Dequeasm's label system:<br>
<code>LabelName:</code> is used like in assembly to define the start of a label. The address can then pushed back onto the queue with <code>~PSH LabelName</code> and then jumped to with any jump operation such as <code>~JMP</code>
''NOTE: All instructions involving multiple values such as <code>SUB</code> will execute in the following order: <code>POP a; POP b; PUSH b - a;</code>''
{| class="wikitable"
! colspan="3" | Queue Manipulation Operations
|-
! Command
! Name
! Action
|-
| style="text-align:center" | <code>POP</code>
| style="text-align:center" | Pop
| Remove element from the dequeue at the specified end of the stack.
|-
| style="text-align:center" | <code>PSH</code>
| style="text-align:center" | Push
| Insert element to the deque at the specified end of the stack.
|-
| style="text-align:center" | <code>DUP</code>
| style="text-align:center" | Duplicate
| Duplicate the value on the specified end of the stack.
|-
| style="text-align:center" | <code>SWP</code>
| style="text-align:center" | Swap
| Swap the positions of the top two values on the stack.
|-
| style="text-align:center" | <code>OVR</code>
| style="text-align:center" | Over
| Put the second item on the specified end stack on the end of the stack WITHOUT removing it from its original location.
|-
| style="text-align:center" | <code>RCW</code>
| style="text-align:center" | Rotate Clockwise
| Rotate the top three items on the stack clockwise on.
|-
| style="text-align:center" | <code>RCC</code>
| style="text-align:center" | Rotate Counterclockwise
| Rotate the top three items on the stack counterclockwise.
|-
| style="text-align:center" | <code>ROL</code>
| style="text-align:center" | Roll
| Move the value from the specified end of the queue to the other end.
|-
|-
| style="text-align:center" | <code>SHL</code>
| style="text-align:center" | Shift left
| Shifts the entire queue left
|-
| style="text-align:center" | <code>SHR</code>
| style="text-align:center" | Shift Right
| Shifts the entire queue right
|-
! colspan="5" | Arithmatic Operations
|-
! Command
! Name
! Action
|-
| style="text-align:center" | <code>ADD</code>
| style="text-align:center" | Addition
| Pushes the sum of the top two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>SUB</code>
| style="text-align:center" | Substraction
| Pushes the difference of the top two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>MUL</code>
| style="text-align:center" | Multiplication
| Pushes the product of the top two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>DIV</code>
| style="text-align:center" | Integer Division
| Pushes the quotient the top two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>MOD</code>
| style="text-align:center" | Modulation
| Pushes the remainder the top two values from the end of the stack to the same end.
|-
! colspan="5" | Logical Operations (0 as False, Non 0 as True)
|-
! Command
! Name
! Action
|-
| style="text-align:center" | <code>AND</code>
| style="text-align:center" | Logical And
| Pushes the result of a logical AND of the first two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>OR</code>
| style="text-align:center" | Logical Or
| Pushes the result of a logical OR of the first two values from the end of the stack to the same end.
|-
| style="text-align:center" | <code>XOR</code>
| style="text-align:center" | Logical Exclusive Or
| Pushes the result of a logical AND of the first two values from the end of the stack to the same end.
|-
! colspan="5" | Flow Control
|-
! Command
! Name
! Action
|-
| style="text-align:center" | <code>JNZ</code>
| style="text-align:center" | Jump if non-zero
| Sets the instruction pointer to the address on the end of the stack if the second value before it is non-zero
|-
| style="text-align:center" | <code>JMP</code>
| style="text-align:center" | Unconditional Jump
| Sets the instruction pointer to the address on the end of the stack.
|-
| style="text-align:center" | <code>JE</code>
| style="text-align:center" | Jump if equal
| Sets the instruction pointer to the address on the end of the stack if the two values before it are equal
|-
| style="text-align:center" | <code>JG</code>
| style="text-align:center" | Jump if greater than
| Sets the instruction pointer to the address on the end of the stack if the second value is greater than the third.
|-
| style="text-align:center" | <code>JL</code>
| style="text-align:center" | Jump if less than
| Sets the instruction pointer to the address on the end of the stack if the second value is less than the third.
|-
| style="text-align:center" | <code>JGE</code>
| style="text-align:center" | Jump if greater than or equal to
| Sets the instruction pointer to the address on the end of the stack if the second value is greater than or equal to the third.
|-
| style="text-align:center" | <code>JLE</code>
| style="text-align:center" | Jump if less than or equal to
| Sets the instruction pointer to the address on the end of the stack if the second value is less than or equal to the third.
|-
! colspan="5" | Input / Output
|-
! Command
! Name
! Action
|-
| style="text-align:center" | <code>OUT</code>
| style="text-align:center" | Output
| Pops a value from an end of the stack and prints it's ascii code as a character.
|-
| style="text-align:center" | <code>INP</code>
| style="text-align:center" | Input
| Takes a singular character as input and pushes it's ascii value to the specified end of the stack.
|-
| style="text-align:center" | <code>HLT</code>
| style="text-align:center" | Halt
| Breaks the program
|}

