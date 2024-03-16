// Definition for singly-linked list.
public class ListNode(int x)
{
    public int val = x;
    public ListNode next = null;
}

public class Solution
{
    public bool HasCycle(ListNode head) {
        ListNode slow = head;
        ListNode fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
            if (slow == fast) {
                return true;
            }
        }
        return false;
    }
}