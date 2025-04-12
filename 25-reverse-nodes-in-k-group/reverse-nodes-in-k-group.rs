// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Break a list into a k-sized chunk and the rest
        fn break_list(mut node: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>, i32) {
            if node.is_none() {
                return (None, None, 0);
            }

            let mut start = node;
            let mut curr = &mut start;
            let mut count = 0;
            
            // Traverse k nodes while counting
            while count < k && curr.is_some() {
                curr = &mut curr.as_mut().unwrap().next;
                count += 1;
            }
            
            // Extract the rest of the list
            let rest = curr.take();
            
            (start, rest, count)
        }

        // Reverse a linked list and return head and tail
        fn rev_list(mut node: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<*mut ListNode>) {
            let mut prev = None;
            let mut curr = node;
            let mut tail_raw: Option<*mut ListNode> = None;

            while let Some(mut n) = curr {
                if prev.is_none() {
                    // Save raw pointer to first node → will become tail
                    tail_raw = Some(&mut *n as *mut _);
                }
                curr = n.next;
                n.next = prev;
                prev = Some(n);
            }

            (prev, tail_raw)
        }

        // Append a list to a tail using raw pointer
        fn append(tail: Option<*mut ListNode>, node: Option<Box<ListNode>>) {
            if let Some(ptr) = tail {
                unsafe {
                    (*ptr).next = node;
                }
            }
        }

        let mut curr = head;
        let mut data = Vec::new();
        
        // Process the list in k-chunks
        loop {
            let (mut chunk, rest, count) = break_list(curr, k);
            
            if chunk.is_some() {
                if count == k {
                    // We have a full k-group, reverse it
                    data.push(rev_list(chunk));
                } else {
                    // Less than k nodes, keep as is
                    let tail_ptr = if let Some(ref mut c) = chunk {
                        // Traverse to find the last node
                        let mut last = c;
                        while last.next.is_some() {
                            last = last.next.as_mut().unwrap();
                        }
                        Some(&mut **last as *mut ListNode)
                    } else {
                        None
                    };
                    data.push((chunk, tail_ptr));
                }
            }
            
            // Move to the next chunk
            curr = rest;
            
            if curr.is_none() {
                break;
            }
        }
        
        // Connect all chunks together
        let mut data_iter = data.into_iter();
        if let Some((head_val, tail_val)) = data_iter.next() {
            let (mut result_head, mut result_tail) = (head_val, tail_val);
            
            for (node, node_tail) in data_iter {
                append(result_tail, node);
                result_tail = node_tail;
            }
            
            result_head
        } else {
            None
        }
    }
}