// Definition for singly-linked list.
public class ListNode(int val = 0, ListNode next = null) {
    public int val = val;
    public ListNode next = next;
}

public class Solution {
    public ListNode ReverseList(ListNode head) {
        ListNode prev = null;
        ListNode curr = head;
        ListNode next = null;
        
        while (curr != null) {
            next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        return prev;
    }

    public ListNode ReverseList2(ListNode head) {
        if (head == null || head.next == null) return head;
        var newHead = ReverseList(head.next);
        head.next.next = head;
        head.next = null;
        return newHead;
    }
}