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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {

        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{

            let mut prev = None;

            while let Some(mut curr) = head{

                head = curr.next.take();
                curr.next = prev;
                prev = Some(curr);
            }

            prev


        }
        
        //I will break the list

        let mut i = 0 ;

        let mut first = Some(Box::new(ListNode{val:0, next:head}));
        let mut f = &mut first;
        let mut middle = None;
        while let Some(ref mut curr) = f {

            if i == left-1{

                middle = curr.next.take();


            }

            f = &mut curr.next;

            i+=1;

        }

        let mut m = &mut middle;
        let mut j = 0;
        let target = right - left;
        let mut second = None;
        while let Some(ref mut curr) = m{


            if j == target{


                second = curr.next.take();

            }

            m = &mut curr.next;
            j+=1;

        }


        let mut reversed = reverse(middle);

        let mut f = &mut first;

        let mut i = 0;

        while let Some(ref mut curr) = f{

            if curr.next.is_none(){
            
                curr.next = reversed;
                break;

        
            }

            f = &mut curr.next;

        }
        let mut f = &mut first;

        while let Some(ref mut curr) = f{

            if curr.next.is_none(){
            
                curr.next = second;
                break;

        
            }

            f = &mut curr.next;

        }

        first.unwrap().next

    }
}