
// Definition for a binary tree node.
public class TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
{
    public int val = val;
    public TreeNode left = left;
    public TreeNode right = right;
}

public class Solution
{
    public IList<int> PreorderTraversal(TreeNode root) {
        var order = new List<int>();
        var stack = new Stack<TreeNode>();
        if (root != null) stack.Push(root);
        while (stack.Count > 0) {
            var node = stack.Pop();
            order.Add(node.val);
            if (node.right != null) stack.Push(node.right);
            if (node.left != null) stack.Push(node.left);
        }
        return order;
    }
}