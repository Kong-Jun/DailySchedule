# DailySchedule
每日学习实践过程记录

# 7月3日
学习Rust的第一天。读了《通过例子学Rust》前九章和《Rust编程之道》前两章，对Rust的设计目标、编程风格和基本语法有了大概的了解。还有很多疑问，主要集中在所有权上，对Rust的引用和变量感到迷惑。

# 7月4日
读了《Rust编程之道》3，4,5章，对所有权和生命周期有了一定了解，但是还有些模糊，需要进一步加强。 了解了一下cargo。

# 7月5日
复习毛概（毛概真的又多有又难），影响了Rust学习的进度，明天考毛概，考完再追赶Rust的进度。

# 7月6日
考完毛概了。又读了几遍《Rust编程之道》中所有权和生命周期的部分，读了《通过例子学Rust》中Trait、泛型、所有权、生命周期和其他一些零零散散的内容。对于生命周期参数还是有一些迷惑，明白生命周期的概念和它的作用，但是不太明白生命周期参数是怎样推导出来的，尤其是生命周期参数究竟是指引用本身的生命周期，还是被指向的对象的生命周期。另外对于枚举类型，尤其是标准库中定义的枚举Option，Result，只知道它的作用，不太了解它的机制。
# 7月7日
读完了《通过例子学Rust》，读了The Book中关于所有权、生命周期、泛型、Trait、枚举、错误处理的内容。感觉弄明白了之前比较疑惑的内容，比如生命周期参数、枚举等。感觉自己对Rust的核心概念已经比较清楚了，但是对于标准库还很不了解，对Rust的一些工具还不熟练。下一步以练习为主，阅读为辅，在练习中巩固知识。
这周要复习电子线路、数学和操作系统，下周一就能结束所有考试，腾出大量时间学习。

# 7月8日
今天时间都用来准备考试了，晚上读了一下《Rust编程之道》第十章“模块化编程”，只看了cargo和模块部分，感觉Rust确实比C/C++现代很多，有好用的统一的工作流管理工具，不用受MakeFile的折磨。

# 7月9日
抽了半天时间学Rust，读了The Book第6、7、8、13、14、15、18章，完成了rustlings大概一半的内容，放到了[我的rustlings仓库](https://github.com/Kong-Jun/rustlings)上。

今天总算是明白了和引用相关的操作和模式匹配。

把`git pull`下来的 rustlings 仓库上传到自己的 github 时遇到了一下困难，感觉要再去学一下 Git 的使用了。

# 7月10日
下午考试，一直复习到了考试前。晚上考完试看了一会Rust。

# 7月11、12日
考试都堆到了13日，所以这两天全用到复习上，只抽了比较短的时间学Rust。

# 7月13日
白天一直在考试，晚上休息了一下，没有学Rust。

# 7月14日
所有课程都考完了，时间多了很多，但是昨晚睡好，上午有点不舒服，下午才开始学习。

尝试用Rust写了一些程序，做了一些rustling练习。感觉对错误处理理解的很浅，很难理解组合子，对Rust风格的错误处理不太习惯，读了两三遍《Rust编程之道》错误处理部分，敲了一边实例代码，感觉清楚了很多。《Rust编程之道》里给出了实际的运用场景和示例程序，根据知识的推进不断重构示例程序，里面的写法非常值得学习。


读了一些*RISC-V Reader*，不太了解计算机组成原理，能感觉到一些 RISC-V 的不同之处，但是没法理解它在技术上的优势。希望将来学习了计算机组成原理后能给出自己的判断和理解。

# 7月15日
搭建了 rcore 实验的环境，完成了 lab 0 的内容。

做 lab 0 的过程中遇到了大量的 Rust 属性，我对属性并不陌生，但是对于 Rust 究竟提供了多少具有实际功能的属性并不了解，有空要去浏览一下常用的属性。

遇到了 LLVM 的内联汇编，感觉跟 GCC 的内联汇编差别不大，将来做实验的过程中再做进一步了解。

没有成功编译出目标架构是 RISC-V 的 GDB,需要进一步查阅资料解决这件事，没有调试器怎么活啊。

# 7月16日
用 Rust 尝试做了一些和“链接”有关的练习，比如链表之类的，加强对所有权的理解和 Rust 编程的能力。写程序的过程中遇到了不少障碍，感觉 Rust 一碰到链式结构就变的复杂起来，所有的 Rust 知识都要用到，尤其是要一直考虑所有权的问题，有不小的心智负担。接下来的 Rust 练习应该集中在自己比较熟悉的常见数据结构和算法，确保既能使用到 Rust 特性又不会带来语言之外的难度。

做完了 rustlings 中除了 *conversion*、*standard_library_type* 和 *generics* 外的全部练习。*conversion* 和 *standard_library_type* 涉及不少标准库的东西，先放一放，对 Rust 理解更深了在做。*generics* 里有两道题不会做，感觉这两道题挺深的，明天查阅资料解决它们。	

**TODO**:

1. 把所有和项目相关的 repo 加入到这个 repo 中，方便查看。先把 rustlings 练习的 repo 搬运过来。
2. 向其他同学一样设置一个可以跳转到每天的记录的表格。
