//! 自己实现的基于线段树的分配器[`SegmentTreeAllocator]

use super::Allocator;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::mem;
use core::option::Option::{Some, None};

// const MAX_PAGE_NUMBER: usize = 128 * 1024 * 1024 / 4096;
pub struct SegmentTreeAllocator {
    vec: Vec<usize>,
    capacity: usize,
}

#[inline(always)]
fn left(node: usize) -> usize {
    2 * node + 1
}

#[inline(always)]
fn right(node: usize) -> usize {
    2 * node + 2
}

#[inline(always)]
fn mid(start: usize, end: usize) -> usize {
    (start + end) / 2
}

fn build_segment_tree(vec: &mut Vec<usize>, root: usize, start: usize, end: usize) {
    assert!(start <= end);

    if start == end {
        vec[root] = 1;
    } else {
        build_segment_tree(vec, left(root), start, mid(start, end));
        build_segment_tree(vec, right(root), mid(start, end) + 1, end);

        vec[root] = vec[left(root)] + vec[right(root)];
    } 

}

fn get_offset(vec: &Vec<usize>, mut root: usize, mut start: usize, mut end: usize) -> usize {
    assert!(start <= end);
    loop {
        if start == end {
            break;
        }

        if vec[left(root)] >= 1 {
            root = left(root);
            end = mid(start, end);
        } else {
            root = right(root);
            start = mid(start, end) + 1;
        }
    }

    start
}

fn update_segment_tree(vec: &mut Vec<usize>, root: usize, start: usize, end: usize, offset: usize, val: usize) {
    assert!(start <= end);
    if start == end {
        assert!(vec[root] == 1 || vec[root] == 0);
        vec[root] = val;
    }
    else {
        if offset <= mid(start, end) {
            update_segment_tree(vec, left(root), start, mid(start, end), offset, val);
        }
        else {
            update_segment_tree(vec, right(root), mid(start, end) + 1, end, offset, val);
        }
            vec[root] = vec[left(root)] + vec[right(root)];
    }

}
impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        // 直接 push 上万个元素影响性能。
        // 我想在堆上分配数组，然后通过 unsafe 方法将其转化为 Vec，怎么做？？
        // 在文档中看到 Vec<T> 实现了 From<Box<[T]>> Trait，尝试使用 Box 在堆上分配 数组，再利用 From Trait 转化为 Vec。
        // 但是这样行不通，数组的大小在运行时才能确定，而 Rust 不支持动态数组，我不知道怎样像 C 那样利用 malloc() 分配一段内存作为数组。rCore 支持的最大内存是已知的（128M)，所以分配一个大小足够记录所有页的数组。
        // let i = 0;
        // debug_assert_eq!(i, 1, "before allocate array");
        // let array: Box<[usize]> = Box::new([0; 2 * MAX_PAGE_NUMBER - 1]);
        // debug_assert_eq!(array.len(), 2 * MAX_PAGE_NUMBER, "failed to allocate array");
        // debug_assert_eq!(array[0], 0, "failed to allocate array");
        // for (i, val) in array.iter().enumerate() {
        //     if i % 100 == 0 {
        //         println!("{}: {}", i, val);
        //     }
        // }
        // let mut vec = Vec::from(array);

		let layout:Layout = unsafe {
			Layout::from_size_align_unchecked(3 * capacity * mem::size_of::<usize>(), mem::size_of::<usize>())
		};

		// 分配内存
		let ptr: *mut usize = unsafe {
			alloc::alloc::alloc(layout) as *mut usize
		}; 
		let mut vec = unsafe {
			Vec::from_raw_parts(ptr, 3 * capacity, 3 * capacity)
		};
        
        build_segment_tree(&mut vec, 0, 0, capacity - 1);
        Self {
            vec: vec,
            capacity: capacity,
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        let result;
        result = if self.capacity == 0 || self.vec[0] == 0 {
            None
        } else {
            Some(get_offset(&self.vec, 0, 0, self.capacity -1))
        };
        if let Some(offset) = result {
            update_segment_tree(&mut self.vec, 0, 0, self.capacity - 1, offset, 0);
        }
        result
    }

    fn dealloc(&mut self, index: usize) {
        update_segment_tree(&mut self.vec, 0, 0, self.capacity - 1, index, 1);
    }
}

