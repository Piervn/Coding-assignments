// Definition for a binary tree node.
public class TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
{
    public int val = val;
    public TreeNode left = left;
    public TreeNode right = right;
}

public class Solution
{
    public TreeNode InvertTree(TreeNode root) {
        if (root is null) return null;
        (root.left, root.right) = (InvertTree(root.right), InvertTree(root.left));
        return root;
    }
}