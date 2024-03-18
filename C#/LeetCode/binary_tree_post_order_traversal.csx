// Definition for a binary tree node.
using Microsoft.CodeAnalysis.CSharp.Syntax;

public class TreeNode {
    public int val;
    public TreeNode left;
    public TreeNode right;
    public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

public class Solution
{
    public IList<int> PostorderTraversal(TreeNode root) {
        var order = new List<int>();
        var stack = new Stack<TreeNode>();
        if (root != null) stack.Push(root);
        while (stack.Count > 0) {
            var node = stack.Pop();
            order.Add(node.val);
            if (node.left != null) stack.Push(node.left);
            if (node.right != null) stack.Push(node.right);
        }
        order.Reverse();
        return order;
    }
}