// Definition for singly-linked list.
public class ListNode(int x) {
    public int val = x;
    public ListNode next;
}
 
public class Solution
{
    public ListNode GetIntersectionNode(ListNode headA, ListNode headB) {
        var a = headA;
        var b = headB;
        while (a != b) {
            a = a == null ? headB : a.next;
            b = b == null ? headA : b.next;
        }
        return a;
    }
}