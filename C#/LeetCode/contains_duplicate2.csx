public class Solution
{
    public bool ContainsNearbyDuplicate(int[] nums, int k) {
        if (k == 0) return false;
        HashSet<int> set = [];
        for (int i = 0; i < nums.Length; i++) {
            if (set.Count > k) set.Remove(nums[i - k - 1]);
            if (!set.Add(nums[i])) return true;
        }
        return false;
    }
}