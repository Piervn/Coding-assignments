// Definition for a binary tree node.
public class TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
{
    public int val = val;
    public TreeNode left = left;
    public TreeNode right = right;
}

public class Solution
{
    int IntPow(int x, uint pow) {
        int ret = 1;
        while (pow != 0) {
            if ((pow & 1) == 1)
                ret *= x;
            x *= x;
            pow >>= 1;
        }
        return ret;
    }

    public uint GetHeightOfCompleteTree(TreeNode root) {
        return (root == null ? 0 : 1 + GetHeightOfCompleteTree(root.left));
    }

    // Complexity: O((log n)^2)
    public int CountNodes(TreeNode root) {
        if (root == null) return 0;
        if (root.left == null) return 1;
        if (root.right == null) return 2;
        uint leftH = GetHeightOfCompleteTree(root.left);
        uint rightH = GetHeightOfCompleteTree(root.right);
        if (leftH == rightH) return CountNodes(root.right) + IntPow(2, leftH);
        else return CountNodes(root.left) + IntPow(2, rightH);
    }
}