// Definition for singly-linked list.
public class ListNode(int val = 0, ListNode next = null)
{
    public int val = val;
    public ListNode next = next;
}

public class Solution
{
    public ListNode RemoveElements(ListNode head, int val) {
        var prehead = new ListNode(0, head);
        var iter = prehead;
        while (iter.next != null) {
            if (iter.next.val == val) iter.next = iter.next.next;
            else iter = iter.next;
        }
        return prehead.next;    
    }
}