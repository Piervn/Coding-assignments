using System.ComponentModel.Design;
using System.Runtime.Serialization.Formatters;

public class Solution
{
    public bool ContainsDuplicate(int[] nums) {
        HashSet<int> set = [];
        foreach (int num in nums) {
            if (set.Contains(num)) return true;
            set.Add(num);
        }
        return false;
    }
}