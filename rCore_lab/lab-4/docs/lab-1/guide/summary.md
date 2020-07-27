## 小结

本章完成了 RISC-V 中有关中断处理的部分，我们实现了中断相关的上下文保存和切换，使得原来正在的运行的程序不需要做任何处理就可以让操作系统处理好中断或异常。我们进一步完成了简单的断点中断和时钟中断，展示了中断处理的执行过程，为后面的章节（包括系统调用的处理）打下了一定的基础。

在下一章节中，我们将从物理内存的管理出发，让操作系统真正可以去管理我们的可以使用的内存。