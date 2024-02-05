#include <iostream>
using namespace std;
// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  bool hasPathSum(TreeNode *root, int targetSum) {
    if (root == nullptr) return false;
    if (root->left == nullptr && root->right == nullptr) {
      return targetSum == root->val;
    }
    return hasPathSum(root->left, targetSum - root->val) || hasPathSum(root->right, targetSum - root->val);
  }
};

int main() {
  TreeNode *root = new TreeNode(5);
  root->left = new TreeNode(4);
  root->right = new TreeNode(8);
  root->left->left = new TreeNode(11);
  root->left->left->left = new TreeNode(7);
  root->left->left->right = new TreeNode(2);
  root->right->left = new TreeNode(13);
  root->right->right = new TreeNode(4);
  root->right->right->right = new TreeNode(1);
  Solution s;
  string result = s.hasPathSum(root, 22) ? "Correct" : "Incorrect";
  cout << result << endl;
  return 0;
}